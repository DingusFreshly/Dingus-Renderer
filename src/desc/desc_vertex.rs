use wgpu::{VertexFormat, VertexStepMode};
use wgpu;
use std::convert::From;
///describes a property about a vertex, such as position, colour,
#[derive(Hash)]
pub struct VertexAttribute {
    ///what slot in the shader the attribute is mapped to, gpu capability limited
    shader_location: u32,
    ///https://docs.rs/wgpu/latest/wgpu/enum.VertexFormat.html
    format: VertexFormat,
    ///its offset in memory in bytes, use VertexLayout to calculate
    offset: u64
}
impl From<&VertexAttribute> for wgpu::VertexAttribute {
    fn from(value: &VertexAttribute) -> Self {
        wgpu::VertexAttribute {
            shader_location: value.shader_location,
            format: value.format,
            offset: value.offset
        }
    }
}

///describes the layout of one vertex buffer, what attributes the vertex has and their locations
#[derive(Hash)]
pub struct VertexBufferLayout {
    pub(crate) array_stride : u64,
    step_mode: VertexStepMode,
    attributes: Vec<VertexAttribute>,
}
impl VertexBufferLayout {
    pub fn to_wgpu_attrs(&self) -> Vec<wgpu::VertexAttribute> {
        self.attributes.iter().map(|attr| attr.into()).collect()
    }
}

pub struct VertexLayout();

impl VertexLayout {
    const POSITION : usize = size_of::<[u32 ; 3]>();
    
}