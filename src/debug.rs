use std::collections::HashMap;
use wgpu::wgt::{BufferDescriptor, QuerySetDescriptor};

/// Per-frame rendering statistics. Reset at the start of every Renderer::render().
#[derive(Clone, Debug, Default)]
pub struct DebugStats {
    /// Draw call count that frame
    pub draw_calls:           u32,
    pub compute_dispatches:   u32,
    ///
    pub bytes_uploaded:       u64,

    pub pipeline_binds:       u32,
    /// Time the gpu took to execute one frame
    pub gpu_frame_time_ms:    f32,
    /// Per pass GPU timings. Added one frame late.
    pub pass_timings:         HashMap<&'static str, f32>,
    pub buffer_memory_bytes:  u64,
    /// How much memory all the textures take up.
    pub texture_memory_bytes: u64,
    ///
    pub pending_destructions: u32,
}

impl DebugStats {
    /// Resets the depug stats, called per frame in `Renderer`.
    pub fn reset(&mut self) {
        self.draw_calls          = 0;
        self.compute_dispatches  = 0;
        self.bytes_uploaded      = 0;
        self.pipeline_binds      = 0;
        self.gpu_frame_time_ms   = 0.0;
        self.pending_destructions = 0;
        // pass_timings intentionally not reset, it holds the previous frame's results and it overwritten.
    }
}

/// GPU timestamp queries for per pass timing.
/// Requires `wgpu::Features::TIMESTAMP_QUERY`.
pub struct TimestampQuerySet {
    /// Query set holds a mutable storage for the results of queries.
    /// Queries: small pieces of information extracted from other operations such as render passes
    query_set:        wgpu::QuerySet,
    resolve_buffer:   wgpu::Buffer,
    readback_buffer:  wgpu::Buffer,
    /// pass name -> (begin_slot_index, end_slot_index)
    slots:            HashMap<&'static str, [u32; 2]>,
    /// Amount of slots + 1
    next_slot:        u32,
    /// Nanoseconds per GPU tick. Multiply delta_ticks * period to get nanoseconds.
    timestamp_period: u128,
}

impl TimestampQuerySet {
    pub fn new(device: &wgpu::Device, adapter: &wgpu::Adapter, max_passes: u32) -> Self {
        //one for start timestamp, and end timestamp
        let count =     max_passes * 2;
        let byte_size = count as u64 * 8;
        let timestamp_period = adapter.get_presentation_timestamp().0;

        let query_set = device.create_query_set(&QuerySetDescriptor {
            label:  Some("timestamp_queries"),
            ty:     wgpu::QueryType::Timestamp,
            count:  max_passes//TODO
        });

        let resolve_buffer = device.create_buffer(&BufferDescriptor {
            label:  Some("timestamp_resolve"),
            size:   byte_size,
            usage:  wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: true
        });

        let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label:              Some("timestamp_readback"),
            size:               byte_size,
            usage:              wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            query_set, resolve_buffer, readback_buffer,
            slots: HashMap::new(), next_slot: 0,timestamp_period
        }
    }
    /// Allocate begin/end timestamp slots for a named pass. Call at setup time.
    pub fn register_pass(&mut self, name: &'static str) {
        let begin = self.next_slot;
        self.slots.insert(name, [begin, begin + 1]);
        self.next_slot += 2;
    }
    /// Timestamp begins
    pub fn write_begin(&self, enc: &mut wgpu::CommandEncoder, name: &'static str) {
        if let Some(&[begin, _]) = self.slots.get(name) {
            enc.write_timestamp(&self.query_set, begin);
        }
    }
    /// Timestamp ends
    pub fn write_end(&self, enc: &mut wgpu::CommandEncoder, name: &'static str) {
        if let Some(&[_, end]) = self.slots.get(name) {
            enc.write_timestamp(&self.query_set, end);
        }
    }

    /// Resolve all timestamp values into the GPU-side resolve buffer.
    /// Append to the command encoder before submitting.
    pub fn resolve(&self, enc: &mut wgpu::CommandEncoder) {
        if self.next_slot == 0 { return; }
        enc.resolve_query_set(&self.query_set, 0..self.next_slot, &self.resolve_buffer, 0);
    }
    /// Read timing results from the PREVIOUS frame (one frame of readback latency is normal).
    /// Returns a map of pass name -> GPU execution time in milliseconds.
    pub fn read_results(&self, device: &wgpu::Device) -> HashMap<&'static str, f32> {
        let mut out = HashMap::new();
        let slice = self.readback_buffer.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });

        let data       = slice.get_mapped_range();
        let timestamps: &[u64] = bytemuck::cast_slice(&data);

        for (&name, &[begin, end]) in &self.slots {
            if end as usize >= timestamps.len() { continue; }
            let delta_ticks = timestamps[end as usize].saturating_sub(timestamps[begin as usize]) as u128;
            let ns  = delta_ticks * self.timestamp_period;
            out.insert(name, (ns / 1_000_000) as f32);
        }

        drop(data);
        self.readback_buffer.unmap();
        out
    }
}