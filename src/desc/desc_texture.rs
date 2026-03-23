use std::cmp::{max, max_by, min};
use wgpu::{TextureFormat, TextureUsages, TextureDimension};

///Describes a texture on the gpu
#[derive(Copy, Clone)]
pub struct TextureDesc {
    
    label: Option<&'static str>,

    ///texel width
    width: u32,
    ///texel height
    height: u32,
    /// If dimension == D3: 
    ///     This represents the 3d depth of the texture
    ///     Texture will be `width * height * depth_or_layers`
    ///If dimension == D2:
    ///     This represents how many layers of 2D images exist
    ///     Will be `depth_or_layers` amount of `width * height` images
    ///     For a cube, this might be 6 because it has 6 texture faces
    ///     If `depth_or_layers == 1`, its just a normal 2D texture
    depth_or_layers: u32,
    ///Mipmapping is a technique used for efficiently rendering far-away textures
    ///for each mip-map level, it creates a smaller texture that is N^0.5 smaller in resolution
    /// when the gpu wants to render a texture that has a smaller resolution than whats on the screen, 
    /// it calculates the pixel ratio and looks for the correct mip map level to use
    /// *more levels == more ram but better performance for far or small textures*
    /// mip_levels == 1 for no mipmap
    mip_levels: u32,
    ///how the texture pixel data is stored in numbers
    /// https://docs.rs/wgpu/latest/wgpu/enum.TextureFormat.html
    format: TextureFormat,
    ///https://docs.rs/wgpu/latest/wgpu/struct.TextureUsages.html
    usage: TextureUsages,
    ///D2 or D3
    dimension: TextureDimension,
    ///levels for multi-sample anti aliasing
    /// - MSAA smoothes out jagged pixel corners by applying lighter colour pixels around.
    /// - controls the amount of samples per pixel stored.
    /// - increases memory usage, and improves image quality 
    /// - 1 to turn off
    sample_count: u32,
}

impl TextureDesc {
    ///gets how many mip map levels will fit with
    pub fn full_mip_levels(&self) ->  u32{
        
        f32::floor(f32::log2(f32::max(self.height as f32, self.width as f32))) as u32
         
    }
    ///calculates size of all mipmap levels
    pub fn size_bytes(&self) -> u64 {
        let m : u64 = min(self.full_mip_levels(), self.mip_levels) as u64;
        let mut size : u64 = 0;
        
        for i in 0..m {
            size += max(self.width as u64 >> i , 1) * max(self.height as u64 >> i, 1);
        }
        size
    }
    
    pub fn to_wgpu(&self) -> wgpu::TextureDescriptor {
        wgpu::TextureDescriptor {
            
            label: self.label,
            
            size: wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: self.depth_or_layers,
            },
            
            mip_level_count: self.mip_levels,
            sample_count: self.sample_count,
            dimension: self.dimension,
            format: self.format,
            usage: self.usage,
            
            view_formats: &[],
        }
    }
}

