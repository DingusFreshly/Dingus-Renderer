use wgpu::{BufferDescriptor, BufferUsages, Device};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::desc::prelude::VertexBufferLayout;

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer:  Option<wgpu::Buffer>,
    /// Vertex count for non-indexed draws, index count for indexed draws.
    /// how many draw steps the gpu needs to take
    pub element_count: u32,
    pub layout:        VertexBufferLayout,
    pub index_format:  Option<wgpu::IndexFormat>,
    /// Original vertex byte size, used to validate in-place updates.
    pub vertex_bytes:  u64,
}

impl Mesh {
    pub fn create(
        label: Option<&'static str>,
        device: &Device,
        vertices: &[u8],
        indices: Option<&[u32]>,
        layout: VertexBufferLayout
    )-> Self {
        
        let vertex_label = label.map(|s| format!("{}_vertex_buffer", s));

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: vertex_label.as_deref(),
            contents: vertices,
            usage: BufferUsages::VERTEX |BufferUsages::COPY_DST
        });
        let vertex_bytes = vertices.len() as u64;

        let (index_buffer, element_count, index_format) = match indices {
            Some(idx) => {//we have an indexed buffer
                let index_label = label.map(|s| format!("{}_index_buffer", s));

                let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
                    label:index_label.as_deref(),
                    contents: bytemuck::cast_slice(idx),
                    usage: BufferUsages::INDEX | BufferUsages::COPY_DST
                });
                
                let count = idx.len() as u32;
                //indexes are stored as u32
                let format = wgpu::IndexFormat::Uint32;
                
                (Some(index_buffer), count, Some(format))
            },
            None => {//we dont have an index buffer
                let count = vertices.len() as u32 / layout.array_stride as u32;

                (None, count, None)
            }
        };
        
        Self {vertex_buffer, index_buffer, element_count, layout, index_format, vertex_bytes}
    }
}