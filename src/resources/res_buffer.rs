use wgpu::Buffer;
use crate::desc::prelude::BufferDesc;
use std::sync::Arc;

pub struct GpuBuffer {
    pub buffer: Buffer,
    pub desc: BufferDesc,
    pub size_bytes: u64
}

impl GpuBuffer {
    pub fn create(device: &wgpu::Device, desc: BufferDesc) -> Self {
        let size_bytes = desc.size;
        let buffer     = device.create_buffer(&desc.to_wgpu());
        Self { buffer, desc, size_bytes }
    }
}

///cached description of how group of buffers, textures or samplers map to shaders
pub struct CachedBindGroup {
    ///group of buffers, textures or samplers bound to shaders
    pub bind_group: wgpu::BindGroup,
    pub layout:     Arc<wgpu::BindGroupLayout>,
    /// Hash of the BindGroupDesc used to create this. Used for cache lookup.
    pub desc_hash:  u64,
}

