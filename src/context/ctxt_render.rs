use std::sync::Arc;
use wgpu::{Instance, PollError, PollStatus};
use crate::error::RendererError;

pub const POLL_WAIT_TIME : std::time::Duration = std::time::Duration::from_secs(5);

/// The GPU device and queue, shared via Arc across all renderer subsystems.
pub struct RenderContext {
    pub instance: Instance,
    pub device:   wgpu::Device,
    pub queue:    wgpu::Queue,
    pub adapter:  wgpu::Adapter,
    pub features: wgpu::Features,
    pub limits:   wgpu::Limits,
    pub backend:  wgpu::Backend,
}

impl RenderContext {
    /// Initialize adapter, device, and queue, async because adapter selection
    /// involves asking the OS driver which GPU is available
    pub async fn new(backends: wgpu::Backends) -> Result<Arc<Self>, RendererError> {
        //TODO! figure out what the fuck the other parameters do
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::HighPerformance,
                //TODO! create a temporary surface to satisfy this parameter so it works on WebGpu and phone targets
                compatible_surface:     None,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|x| RendererError::NoAdapter(backends))?;

        let features = adapter.features();
        let limits = adapter.limits();
        let backend = adapter.get_info().backend;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    //TODO! label properly
                    label:             Some("renderer_device"),
                    required_features: features,
                    required_limits:   limits.clone(),
                    memory_hints:      wgpu::MemoryHints::Performance,
                    ..Default::default()
                },
            )
            .await?;

        log::info!("GPU: {:?} via {:?}", adapter.get_info().name, backend);
        
        Ok(Arc::new(Self { instance, device, queue, adapter, features, limits, backend }))
    }
    /// Submit commands to a one-shot encoder. Blocks until the GPU finishes
    /// Used for initialization work (mip generation, initial uploads)
    /// F takes in a command encoder, and can encode comamnds such as RenderPass, ComputePass, etc
    pub fn submit_immediate<F: FnOnce(&mut wgpu::CommandEncoder)>(&self, f: F) -> Result<PollStatus, PollError> {
        let mut enc = self.device.create_command_encoder(
            //TODO! proper label
            &wgpu::CommandEncoderDescriptor { label: Some("immediate") }
        );
        f(&mut enc);
        //queue.submit takes in an iterator, but we only do it for one in this situation
        let idx = self.queue.submit(std::iter::once(enc.finish()));

        let wait_mode = wgpu::PollType::Wait {
            submission_index: Some(idx),
            timeout: Some(POLL_WAIT_TIME)
        };

        self.device.poll(wait_mode)
    }
    pub fn supports(&self, f: wgpu::Features) -> bool { self.features.contains(f) }
    pub fn backend_name(&self) -> &'static str {
        match self.backend {
            wgpu::Backend::Vulkan       => "Vulkan",
            wgpu::Backend::Metal        => "Metal",
            wgpu::Backend::Dx12         => "DirectX 12",
            wgpu::Backend::Gl           => "OpenGL ES",
            wgpu::Backend::BrowserWebGpu => "WebGPU",
            _                           => "Unknown",
        }
    }
}