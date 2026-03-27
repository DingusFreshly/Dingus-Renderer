use std::ops::Range;
use bytemuck::Pod;
use log::warn;
use winit::window::WindowId;
use crate::desc::prelude::TextureDesc;
use crate::handle::aliases::{BindGroupHandle, BufferHandle, MeshHandle, PipelineHandle, TextureHandle, FULLSCREEN_MESH};
use crate::handle::Handle;
use crate::resources::prelude::PipelineKind;
use crate::sort_key::SortKey;
use crate::types::Rect;
use smallvec::SmallVec;
use winit::dpi::Size;

///The api for the renderer, these commands are sorted by dependency in `RenderGraph` and flushed every frame.
pub enum RenderCommand {
    Draw(DrawCall),
    DrawIndirect {
        pipeline: PipelineHandle,
        mesh: MeshHandle,
        bind_groups: SmallVec<BindGroupHandle, 4>,//TODO! make smallvec with size 4
        push_constants: [u8; 128],
        /// Buffer containing DrawIndirectArgs structs packed sequentially.
        indirect_buffer: BufferHandle,
        indirect_offset: u64,
        draw_count: u32,
        window: WindowId,
        sort_key: SortKey,
    },

    WriteTexture {
        handle: TextureHandle,
        data: Vec<u8>,
        layout: TextureDesc,
        region: Rect
    },
    WriteBuffer {
        handle: BufferHandle,
        offset: u64,
        data: Vec<u8>
    }

}
//TODO! impl default
pub struct DrawCall {
    pipeline: PipelineHandle,
    mesh: MeshHandle,
    bind_groups: Vec<BindGroupHandle>,//TODO! make smallvec with size 4
    push_constants: [u8; 128],
    instances: Range<u32>,
    window: WindowId,
    pub(crate) sort_key: SortKey,
    scissor: Option<Rect>
}

impl DrawCall {
    const PUSH_CONSTANT_BYTE_SIZE : usize = 128;
    pub fn with_push_constants<T: Pod>(mut self, value: &T) -> Self {
        let bytes =bytemuck::bytes_of(value);
        let n = bytes.len();
        if n > Self::PUSH_CONSTANT_BYTE_SIZE {
            warn!("Push constant exceeded {} bytes! Truncation will occur.", Self::PUSH_CONSTANT_BYTE_SIZE)
        }
        self.push_constants[..n].copy_from_slice(&bytes[..n]);

        self
    }
}

impl Default for DrawCall {
    fn default() -> Self {
        Self {
            pipeline: Handle::null(),
            mesh: FULLSCREEN_MESH,
            bind_groups: Vec::new(),
            push_constants: [0; 128],
            instances: 0..1,
            sort_key: 0,
            window: WindowId::dummy(),
            scissor: None
        }
    }
}
