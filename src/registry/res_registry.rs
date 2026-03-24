use crate::resources::prelude::*;
use crate::slotmap::SlotMap;
use crate::handle::{Handle,aliases::*};
use std::collections::HashMap;
use crate::desc::prelude::{BindGroupDesc, BindingResource};
use crate::error::RendererError;

/// Owns every GPU resource. All external access goes through typed handles.
pub struct ResourceRegistry {
    textures:    SlotMap<TextureHandle,    Texture>,
    buffers:     SlotMap<BufferHandle,     GpuBuffer>,
    pipelines:   SlotMap<PipelineHandle,   Pipeline>,
    meshes:      SlotMap<MeshHandle,       Mesh>,
    samplers:    SlotMap<SamplerHandle,    Sampler>,
    bind_groups: SlotMap<BindGroupHandle,  CachedBindGroup>,
    shaders:     SlotMap<ShaderHandle,     ShaderModule>,

    /// hash(PipelineDesc) → handle : prevents duplicate pipeline compilation.
    pipeline_cache:   HashMap<u64, PipelineHandle>,
    /// hash(BindGroupDesc) → handle : prevents duplicate descriptor allocation.
    bind_group_cache: HashMap<u64, BindGroupHandle>,
    /// hash(shader source) → handle : prevents duplicate shader compilation.
    shader_cache:     HashMap<u64, ShaderHandle>,

    /// Wgpu objects waiting to be dropped after the current frames GPU work finishes.
    deferred_textures: Vec<wgpu::Texture>,
    deferred_buffers:  Vec<wgpu::Buffer>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        //jarvis, format this for me please
        Self {
            textures:         SlotMap::new(),
            buffers:          SlotMap::new(),
            pipelines:        SlotMap::new(),
            meshes:           SlotMap::new(),
            samplers:         SlotMap::new(),
            bind_groups:      SlotMap::new(),
            shaders:          SlotMap::new(),
            pipeline_cache:   HashMap::new(),
            bind_group_cache: HashMap::new(),
            shader_cache:     HashMap::new(),
            deferred_textures: Vec::new(),
            deferred_buffers:  Vec::new(),
        }
    }
    // ── Insert ────────────────────────────────────────────────────────────

    pub fn insert_texture(&mut self, t: Texture)             -> TextureHandle   { self.textures.insert(t) }
    pub fn insert_buffer(&mut self, b: GpuBuffer)            -> BufferHandle    { self.buffers.insert(b) }
    pub fn insert_pipeline(&mut self, p: Pipeline)           -> PipelineHandle  { self.pipelines.insert(p) }
    pub fn insert_mesh(&mut self, m: Mesh)                   -> MeshHandle      { self.meshes.insert(m) }
    pub fn insert_sampler(&mut self, s: Sampler)             -> SamplerHandle   { self.samplers.insert(s) }
    pub fn insert_bind_group(&mut self, bg: CachedBindGroup) -> BindGroupHandle { self.bind_groups.insert(bg) }
    pub fn insert_shader(&mut self, sm: ShaderModule)        -> ShaderHandle    { self.shaders.insert(sm) }

    // ── Lookup ────────────────────────────────────────────────────────────

    pub fn get_texture(&self,    h: TextureHandle)    -> Option<&Texture>         { self.textures.get(h) }
    pub fn get_buffer(&self,     h: BufferHandle)     -> Option<&GpuBuffer>       { self.buffers.get(h) }
    pub fn get_pipeline(&self,   h: PipelineHandle)   -> Option<&Pipeline>        { self.pipelines.get(h) }
    pub fn get_mesh(&self,       h: MeshHandle)       -> Option<&Mesh>            { self.meshes.get(h) }
    pub fn get_sampler(&self,    h: SamplerHandle)    -> Option<&Sampler>         { self.samplers.get(h) }
    pub fn get_bind_group(&self, h: BindGroupHandle)  -> Option<&CachedBindGroup> { self.bind_groups.get(h) }
    pub fn get_shader(&self,     h: ShaderHandle)     -> Option<&ShaderModule>    { self.shaders.get(h) }

    pub fn get_texture_mut(&mut self, h: TextureHandle) -> Option<&mut Texture>   { self.textures.get_mut(h) }
    pub fn get_buffer_mut(&mut self,  h: BufferHandle)  -> Option<&mut GpuBuffer> { self.buffers.get_mut(h) }

    // ── Deferred Destruction ──────────────────────────────────────────────

    pub fn destroy_texture(&mut self, h: TextureHandle) {
        if let Some(t) = self.textures.remove(h) {
            self.deferred_textures.push(t.texture);
        }
    }

    pub fn destroy_buffer(&mut self, h: BufferHandle) {
        if let Some(b) = self.buffers.remove(h) {
            self.deferred_buffers.push(b.buffer);
        }
    }

    pub fn destroy_pipeline(&mut self, h: PipelineHandle) {
        if self.pipelines.remove(h).is_some() {
            self.pipeline_cache.retain(|_, v| *v != h);
        }
    }

    pub fn destroy_mesh(&mut self, h: MeshHandle) {
        if let Some(m) = self.meshes.remove(h) {
            self.deferred_buffers.push(m.vertex_buffer);
            if let Some(ib) = m.index_buffer { self.deferred_buffers.push(ib); }
        }
    }

    pub fn destroy_bind_group(&mut self, h: BindGroupHandle) {
        if self.bind_groups.remove(h).is_some() {
            self.bind_group_cache.retain(|_, v| *v != h);
        }
    }

    pub fn destroy_sampler(&mut self, h: SamplerHandle) {
        self.samplers.remove(h);
    }

    /// Drop all deferred wgpu objects. Call at end of frame after GPU submission.
    pub fn flush_deferred(&mut self) {
        self.deferred_textures.clear();
        self.deferred_buffers.clear();
    }

    // ── Cache Lookups ─────────────────────────────────────────────────────

    pub fn find_pipeline(&self,      hash: u64) -> Option<&PipelineHandle>  { self.pipeline_cache.get(&hash) }
    pub fn cache_pipeline(&mut self, hash: u64, h: PipelineHandle)         { self.pipeline_cache.insert(hash, h); }
    pub fn find_bind_group(&self,    hash: u64) -> Option<&BindGroupHandle> { self.bind_group_cache.get(&hash) }
    pub fn cache_bind_group(&mut self, hash: u64, h: BindGroupHandle)      { self.bind_group_cache.insert(hash, h); }
    pub fn find_shader(&self,        hash: u64) -> Option<&ShaderHandle>    { self.shader_cache.get(&hash) }
    pub fn cache_shader(&mut self,   hash: u64, h: ShaderHandle)           { self.shader_cache.insert(hash, h); }

    // ── Memory Stats ──────────────────────────────────────────────────────
    pub fn buffer_memory_bytes(&self)  -> u64   { self.buffers.values().map(|b| b.size_bytes).sum() }
    pub fn texture_memory_bytes(&self) -> u64   { self.textures.values().map(|t| t.size_bytes).sum() }
    pub fn buffer_count(&self)         -> usize { self.buffers.len() }
    pub fn texture_count(&self)        -> usize { self.textures.len() }
    pub fn pipeline_count(&self)       -> usize { self.pipelines.len() }

    pub fn create_bind_group_inner(
        &self,
        device: &wgpu::Device,
        desc: &BindGroupDesc
    ) -> Result<wgpu::BindGroup, RendererError> {

        let mut wgpu_entries = Vec::with_capacity(desc.entries.len());

        for entry in desc.entries {
            let resource : wgpu::BindingResource = match &entry.resource {
                BindingResource::Buffer {handle, offset, size} => {
                    let buf = self.get_buffer(*handle).ok_or(RendererError::InvalidHandle)?;
                    wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &buf.buffer,
                        offset: *offset,
                        size: size.and_then(std::num::NonZeroU64::new)
                    })
                },
                BindingResource::Sampler(s) => {
                    let smp = self.get_sampler(*s).ok_or(RendererError::InvalidHandle)?;
                    wgpu::BindingResource::Sampler(&smp.sampler)
                }
                BindingResource::Texture { handle, .. } => {
                    let tex = self.get_texture(*handle).ok_or(RendererError::InvalidHandle)?;
                    wgpu::BindingResource::TextureView(&tex.view)
                }
                _ => return Err(RendererError::InvalidHandle)
            };

            wgpu_entries.push(wgpu::BindGroupEntry { binding: entry.binding, resource});
        }

        Ok(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: desc.label,
            layout:desc.layout,
            entries: &wgpu_entries
        }))

    }

}