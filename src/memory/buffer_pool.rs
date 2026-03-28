use std::collections::HashMap;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Label};
use crate::desc::prelude::BufferDesc;

struct BufferPool {

    //Indexed by Powers of 2 sizes ex: 2, 4, 8 are indexs.
    classes: HashMap<u64, Vec<wgpu::Buffer>>,
    usage: wgpu::BufferUsages

}

impl BufferPool {

    pub fn new(usage: wgpu::BufferUsages) -> Self {

        Self { classes: HashMap::new(), usage }

    }

    ///adjust a pre-existing size to a pow of 2 size, always round up
    fn size_classes(size: u64) -> u64 {

        if (size == 0) { return 1; } else {

            size.next_power_of_two() as u64

        }

    }

    ///either create or reuse a previous made buffer in the buffer list based off size
    pub fn aquire(&mut self, min_size: u64, device: Device, label: Option<& str>) -> Buffer {

        let size = Self::size_classes(min_size);
        let matched = self.classes.entry(size).or_insert_with(Vec::new);

        if let Some(buffer) = matched.pop() { return buffer }

        let new_buffer_descriptor: BufferDescriptor = BufferDescriptor {

            label,
            size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_WRITE,
            mapped_at_creation: false

        };

        device.create_buffer(&new_buffer_descriptor)

    }

    ///add buffer into list or buffers (pool)
    pub fn release(&mut self, buffer: Buffer) {

        self.classes.entry(buffer.size()).or_insert_with(Vec::new).push(buffer);

    }

    ///get the total children of buffers under the pool
    pub fn pool_size(&self) -> u64 {

        self.classes.values().map(|v| v.len() as u64).sum()

    }

    ///clear the pool of buffers
    pub fn clear(&mut self) {

        self.classes.clear();

    }

}