use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::fmt;

/// A handle to a wgpu resource, inside the swapchain
/// Indexed by index and generation
/// Type of handles:
pub struct Handle<T> {
    pub(crate) slot_index: usize,
    pub(crate) generation: u32,

    //using T directly would require T to also be Send + Sync, ill make sure of this later
    _marker : PhantomData<fn() -> T>
}
///instead of deriving, we implement raw
/// this is because deriving copy requires T to also be copy, which it is not
/// this is safe because the only data the handle actually carries is copy compatible 
impl<T> Copy  for Handle<T> {}
impl<T> Clone for Handle<T> { fn clone(&self) -> Self { *self } }
impl<T> Eq    for Handle<T> {}
impl<T> PartialEq for Handle<T> {
    fn eq(&self, o: &Self) -> bool {
        self.slot_index == o.slot_index && self.generation == o.generation
    }
}
impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, s: &mut H) {
        self.slot_index.hash(s);
        self.generation.hash(s);
    }
}
impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle<{}>(slot={}, gen={})",
               std::any::type_name::<T>(), self.slot_index, self.generation)
    }
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
pub mod aliases {
    use crate::resources::prelude::*;
    use super::Handle;
    pub type TextureHandle    = Handle<Texture>;
    pub type BufferHandle     = Handle<GpuBuffer>;
    pub type PipelineHandle   = Handle<Pipeline>;
    pub type MeshHandle       = Handle<Mesh>;
    pub type SamplerHandle    = Handle<Sampler>;
    pub type BindGroupHandle  = Handle<CachedBindGroup>;
    pub type ShaderHandle     = Handle<ShaderModule>;

    /// Built-in fullscreen triangle mesh. No vertex buffer needed; clip-space
    /// positions are generated in the vertex shader from `vertex_index`.
    pub const FULLSCREEN_MESH: MeshHandle = MeshHandle::builtin(0);
}
/*

*/