use wgpu::{BufferDescriptor, BufferUsages};

pub struct BufferDesc {
    label: Option<&'static str>,
    ///size in bytes, use `N * std::mem::size_of::<T>() as u64`
    size : u64,
    ///VERTEX, INDEX, UNIFORM, STORAGE, INDIRECT, COPY_SRC, COPY_DST, MAP_READ
    usage: BufferUsages,
    ///if true, the buffer is CPU-writable immediately after creation
    mapped_at_creation: bool
}

impl BufferDesc {
    pub fn to_wgpu(&self) -> BufferDescriptor<'_>{
        BufferDescriptor {
            label: self.label,
            size: self.size,
            usage: self.usage,
            mapped_at_creation: self.mapped_at_creation
        }
    }
}