use crate::handle::aliases::*;
///represents all the resource types and their locations in order
pub struct BindGroupDesc<'a> {
    pub label:   Option<&'static str>,
    pub layout:  &'a wgpu::BindGroupLayout,
    ///
    pub entries: &'a [BindGroupEntry<'a>],
}
///represents a resource type and its location in a buffer
pub struct BindGroupEntry<'a> {
    ///@location of resource in shader
    pub binding:  u32,
    ///type of shader
    pub resource: BindingResource<'a>,
}

///A type of resource a shader can access
pub enum BindingResource<'a> {
    ///Collection of data
    Buffer { handle: BufferHandle, offset: u64, size: Option<u64> },
    ///(handle, offset bytes, size bytes)
    BufferArray(&'a [(BufferHandle, u64, Option<u64>)]),
    ///How to interpret a texture
    Sampler(SamplerHandle),
    ///pixel data
    Texture { handle: TextureHandle, aspect: wgpu::TextureAspect },
    ///
    TextureArray(&'a [TextureHandle]),
}
