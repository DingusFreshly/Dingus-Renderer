///gpu resource for a shader
pub struct ShaderModule {
    ///compiled shader
    pub module:      wgpu::ShaderModule,
    /// Original WGSL source kept for hot-reload support later
    pub source:      String,
    /// Hash of source text, used to detect changes.
    pub source_hash: u64,
    pub label:       Option<&'static str>,
}
impl ShaderModule {
    ///compile source shader and hash
    pub fn from_wgsl(device: &wgpu::Device, source: &str, label: Option<&'static str>) -> Self {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut h = DefaultHasher::new();
        source.hash(&mut h);
        let source_hash = h.finish();
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        Self { module, source: source.to_owned(), source_hash, label }
    }
}