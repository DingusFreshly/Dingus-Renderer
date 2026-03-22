pub struct BindGroupDesc<'a> {
    pub label:   Option<&'static str>,
    pub layout:  &'a wgpu::BindGroupLayout,
    pub entries: &'a [BindGroupEntry],
}

pub struct BindGroupEntry {
    pub binding:  u32,
    //TODO! pub resource: BindingResource<'a>,
}

/*
TODO!pub enum BindingResource<'a> {
    Buffer { handle: BufferHandle, offset: u64, size: Option<u64> },
    ///(handle, offset bytes, size bytes)
    BufferArray(&'a [(BufferHandle, u64, Option<u64>)]),
    Sampler(SamplerHandle),
    Texture { handle: TextureHandle, aspect: wgpu::TextureAspect },
    TextureArray(&'a [TextureHandle]),
}
 */