use crate::desc::prelude::*;
use std::default::Default;

/// A texture on the gpu that lives on the swapchai
pub struct Texture {
    ///the raw GPU texture
    pub(crate) texture: wgpu::Texture,
    ///the default view (all mips, all layers). Used for most shader bindings.
    pub(crate) view: wgpu::TextureView,
    ///the original descriptor, kept for resize logic and tooling
    desc: TextureDesc,
    ///approximate GPU memory usage, tracked for debug stats
    pub(crate) size_bytes: u64
}

impl Texture {
    pub fn create(device: &wgpu::Device, desc: TextureDesc) -> Self {
        let size_bytes = desc.size_bytes();
        let texture = device.create_texture(&desc.to_wgpu());
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        Self { texture, view, desc, size_bytes }
    }

    /// View targeting one mip level and one array layer
    pub fn create_layer_view(&self, mip: u32, layer: u32) -> wgpu::TextureView {
        self.texture.create_view(&wgpu::TextureViewDescriptor {
            base_mip_level:    mip,
            mip_level_count:   Some(1),
            base_array_layer:  layer,
            array_layer_count: Some(1),
            ..Default::default()
        })
    }


    /// View over a contiguous mip range.
    pub fn create_mip_range_view(&self, base_mip: u32, mip_count: u32) -> wgpu::TextureView {
        self.texture.create_view(&wgpu::TextureViewDescriptor {
            base_mip_level:  base_mip,
            mip_level_count: Some(mip_count),
            ..Default::default()
        })
    }

    /// View over one cube face (array layer index 0-5).
    pub fn create_cube_face_view(&self, face: u32) -> wgpu::TextureView {
        self.texture.create_view(&wgpu::TextureViewDescriptor {
            dimension:         Some(wgpu::TextureViewDimension::D2),
            base_array_layer:  face,
            array_layer_count: Some(1),
            ..Default::default()
        })
    }
}