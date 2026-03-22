use std::marker::PhantomData;
/// A handle to a wgpu resource, inside the swapchain
/// Indexed by index and generation
/// Type of handles:
#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug )]
pub struct Handle<T> {
    pub(crate) slot_index: usize,
    pub(crate) generation: u32,

    //using T directly would require T to also be Send + Sync, ill make sure of this later
    _marker : PhantomData<fn() -> T>
}
/// two integers with no interior mutability, so pretty sure this is safe
unsafe impl<T> Sync for Handle<T> {}
unsafe impl<T> Send for Handle<T> {}

impl<T> Handle<T> {
    pub(crate) fn new(slot_index : usize, generation: u32) -> Handle<T> {
        Handle {
            slot_index,
            generation,
            _marker: PhantomData
        }
    }
    /// handle with all values u32::MAX
    pub fn null() -> Handle<T> {
        Handle {
            slot_index: usize::MAX,
            generation: u32::MAX,
            _marker : PhantomData
        }
    }

    pub fn is_null(&self) -> bool {
        self.slot_index == usize::MAX
    }
    ///reserved handles for builtin resources
    /// returns a `Handle` with `generation` 0
    pub const fn builtin(slot_index : usize) -> Handle<T>{
        Handle {
            generation: 0,
            slot_index,
            _marker: PhantomData
        }
    }
 }

//TODO! Add type aliases for all wgpu resources
/*
type TextureHandle    = Handle<Texture>
type BufferHandle     = Handle<GpuBuffer>
type PipelineHandle   = Handle<Pipeline>
type MeshHandle       = Handle<Mesh>
type SamplerHandle    = Handle<Sampler>
type BindGroupHandle  = Handle<CachedBindGroup>
type ShaderHandle     = Handle<ShaderModule>
*/