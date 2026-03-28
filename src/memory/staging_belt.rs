use std::sync::Arc;
use wgpu::{Buffer, CommandEncoder, Device};
use crate::context::RenderContext;
use crate::desc::prelude::TextureDesc;
use crate::handle::aliases::BufferHandle;
use crate::resources::prelude::PipelineKind::Render;

pub struct StagingBelt<'a> {

    device: &'a Device,
    inner : wgpu::util::StagingBelt,
    chunk_size : u64,

}

impl<'a> StagingBelt<'a> {

    pub fn new(device: &'a Device, chunk_size: u64) -> Self {

        Self { device, inner: wgpu::util::StagingBelt::new(device.clone(), chunk_size), chunk_size}

    }

    pub fn write_buffer(&mut self, encoder: &mut CommandEncoder, target: &Buffer, offset: u64, data: &[u8]) {

        let size = std::num::NonZeroU64::new(data.len() as u64).unwrap();
        self.inner.write_buffer(encoder, target, offset, size);

    }

    //WRITE TEXTURE UNAVAILABLE DUE TO FUNCTION NOT EXISTING

    pub fn finish(&mut self) {

        self.inner.finish()

    }

    pub fn recall(&mut self) {

        self.inner.recall()

    }

}