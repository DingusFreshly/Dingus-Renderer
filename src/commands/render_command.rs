use std::ops::Range;
use bytemuck::Pod;
use log::warn;
use winit::window::WindowId;
use crate::desc::prelude::TextureDesc;
use crate::handle::aliases::{BindGroupHandle, BufferHandle, MeshHandle, PipelineHandle, TextureHandle, FULLSCREEN_MESH};
use crate::handle::Handle;
use crate::resources::prelude::{Pipeline, PipelineKind};
use crate::sort_key::SortKey;
use crate::types::{ImageDataLayout, ImageRegion, Rect};
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
    },

    Compute(ComputeCall),

    ComputeIndirect {
        pipeline: PipelineHandle,
        bind_groups: SmallVec<BindGroupHandle, 4>,
        push_constants: [u8; 128],
        /// Buffer containing one DispatchIndirectArgs { x, y, z: u32 }.
        indirect_buffer: BufferHandle,
        indirect_offset: u64,
        label:           Option<&'static str>,
    },

    CopyBufferToTexture {
        buffer:  BufferHandle,
        texture: TextureHandle,
        layout:  ImageDataLayout,
        region:  ImageRegion,
    },
}
//TODO! impl default
pub struct DrawCall {
    pub(crate) pipeline: PipelineHandle,
    pub(crate) mesh: MeshHandle,
    pub(crate) bind_groups: Vec<BindGroupHandle>,//TODO! make smallvec with size 4
    pub(crate) push_constants: [u8; 128],
    pub(crate) instances: Range<u32>,
    window: WindowId,
    pub(crate) sort_key: SortKey,
    pub(crate) scissor: Option<Rect>
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

pub struct ComputeCall {

    pub(crate) pipeline: PipelineHandle,
    pub(crate) bind_groups: SmallVec<BindGroupHandle, 4>,
    pub(crate) push_constants: [u8; 128],
    pub(crate) dispatch: [u32; 3],
    pub(crate) label: Option<&'static str>,

}

impl ComputeCall {

    pub fn with_push_constants<T: Pod>(&self, value: &T) -> Self {

        let bytes =bytemuck::bytes_of(value);
        let n = bytes.len();
        if n > Self::PUSH_CONSTANT_BYTE_SIZE {
            warn!("Push constant exceeded {} bytes! Truncation will occur.", Self::PUSH_CONSTANT_BYTE_SIZE)
        }
        self.push_constants[..n].copy_from_slice(&bytes[..n]);

        self

    }

    fn dispatch_for(n: u32, workgroup_size: u32) -> [u32; 3] {

        let workgroups = n.div_ceil(workgroup_size);
        (workgroups, 1, 1)

    }

    fn dispatch_2d(w: u32, h: u32, wx: u32, wy: u32) -> [u32; 3] {

        let workgroups_x = w.div_ceil(wx);
        let workgroups_y = h.div_ceil(wy);
        (workgroups_x, workgroups_y, 1)

    }

}
