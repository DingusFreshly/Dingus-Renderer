use crate::desc::desc_vertex::VertexBufferLayout;
#[derive(Hash)]
pub struct PipelineDesc<'a> {
    label: Option<&'a str>,
    shader: Option<&'a str>,
    
    //names of functions in shader
    vertex_entry: Option<&'a str>,
    fragment_entry: Option<&'a str>,

    vertex_buffers: &'a [VertexBufferLayout],
    bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
    ///https://docs.rs/wgpu/28.0.0/wgpu/struct.PipelineLayoutDescriptor.html#structfield.immediate_size
    immediate_sizes: u32,
    ///https://docs.rs/wgpu/28.0.0/wgpu/struct.ColorTargetState.html
    colour_targets: &'a [Option<wgpu::ColorTargetState>],
    ///https://docs.rs/wgpu/28.0.0/wgpu/struct.DepthStencilState.html
    depth_stencil: Option<wgpu::DepthStencilState>,
    ///https://docs.rs/wgpu/28.0.0/wgpu/struct.PrimitiveState.html
    primitive: wgpu::PrimitiveState,
    ///https://docs.rs/wgpu/28.0.0/wgpu/struct.MultisampleState.html
    multisample: wgpu::MultisampleState
    
}
