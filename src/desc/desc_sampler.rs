use wgpu::{AddressMode, FilterMode, MipmapFilterMode};
use wgpu::AddressMode::ClampToEdge;

///describes how to read a texture
#[derive(Clone, Copy, Debug)]
pub struct SamplerDesc {
    label: Option<&'static str>,
    ///how textures edges should be handled on u, v, w
    /// https://docs.rs/wgpu/latest/wgpu/enum.AddressMode.html
    address_modes: [AddressMode; 3],
    ///how to filter texture when it needs to be magnified
    mag_filter: FilterMode,
    //how to filter texture when it needs to be minified (made smaller)
    min_filter: FilterMode,
    mipmap_filter: MipmapFilterMode,
    ///Minimum level of detail (mip level) to use
    lod_min_clamp: f32,
    lod_max_clamp: f32,
    ///must be at least 1. If this is not 1, all filter modes must be linear
    anisotropy_clamp: Option<u16>,
}
impl SamplerDesc {
    pub fn linear_clamp() -> Self {
        Self {
            label: Some("linear_clamp"),
            address_modes: [ClampToEdge, ClampToEdge, ClampToEdge],
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: MipmapFilterMode::Linear,
            lod_min_clamp: 0.0, lod_max_clamp: f32::MAX,
            anisotropy_clamp: None,
        }
    }

    pub fn nearest_clamp() -> Self {
        Self {
            label: Some("nearest_clamp"),
            address_modes: [ClampToEdge, ClampToEdge, ClampToEdge],
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::Nearest,
            lod_min_clamp: 0.0, lod_max_clamp: f32::MAX,
            anisotropy_clamp: None,
        }
    }

    pub fn to_wgpu(&self) -> wgpu::SamplerDescriptor<'_> {
        wgpu::SamplerDescriptor {
            label:            self.label,
            address_mode_u:   self.address_modes[0],
            address_mode_v:   self.address_modes[1],
            address_mode_w:   self.address_modes[2],
            mag_filter:       self.mag_filter,
            min_filter:       self.min_filter,
            mipmap_filter:    self.mipmap_filter,
            lod_min_clamp:    self.lod_min_clamp,
            lod_max_clamp:    self.lod_max_clamp,
            compare:          None,
            anisotropy_clamp: self.anisotropy_clamp.unwrap_or(1),
            border_color:     None,
        }
    }
}