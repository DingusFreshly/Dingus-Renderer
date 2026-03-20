use wgpu::*;

#[derive(Debug, Clone, Copy)]
struct Origin3d {

    x : u32,
    y : u32,
    z : u32,

}

impl Origin3d {
    pub const ZERO: Origin3d = Origin3d {x:0,y:0,z:0};
    pub fn new(x:u32,y:u32,z:u32) -> Origin3d { Origin3d {x,y,z} }

    pub fn into_wgpu(o:Origin3d) -> wgpu::Origin3d {
        wgpu::Origin3d {
            x: o.x,
            y: o.y,
            z: o.z
        }
    }
}

impl From<Origin3d> for wgpu::Origin3d {

    fn from(value: Origin3d) -> Self {
        wgpu::Origin3d {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }

}

struct Extent3d {

    width : u32,
    height : u32,
    depth : u32,

}
impl Extent3d {
    pub fn new(width : u32, height : u32, depth_or_array_layers : u32) -> Extent3d {
        Self {
            width, height, depth_or_array_layers
        }
    }

    pub fn new_2d(width : u32, height : u32) -> Self {
        Self { width, height, depth: 1 }
    }

}

impl From<Extent3d> for wgpu::Extent3d {
    
    fn from(value: Extent3d) -> Self {
        wgpu::Extent3d {
            width: value.width,
            height: value.height,
            depth_or_array_layers: value.depth,
        }
    }

}

struct Rect {

    x : u32,
    y : u32,
    width : u32,
    height : u32,

}
impl Rect {
    pub fn new(x : u32, y : u32, width : u32, height : u32) -> Self {
        Self {
            x,
            y,
            width,
            height
        }
    }
    pub fn full(width : u32, height : u32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height
        }
    }

}

struct Viewport {

    x : f32,
    y : f32,
    width : f32,
    height : f32,
    min_depth : f32,
    max_depth : f32,

}

impl Viewport {
    /// a viewport covering the full surface with depth range 0.0 - 1.0
    pub fn full(width : f32, height : f32) -> Self {
        Self { x: 0.0, y: 0.0, width, height, min_depth: 0.0, max_depth: 1.0 }
    }

}

struct ImageDataLayout {

    offset : u64,
    bytes_per_row : Option<u32>,
    rows_per_image : Option<u32>,

}

impl ImageDataLayout {

    pub fn new_2d(offset : u64, bytes_per_row : Option<u32>) -> Self {
        Self { offset, bytes_per_row, rows_per_image: None }
    }

}

struct ImageRegion {

    mip_level : u32,
    origin : Origin3d,
    extent : Extent3d,
    array_layer : u32,

}

impl ImageRegion {

    pub fn full(width : u32, height : u32) -> Self {
        Self { mip_level: 0, origin: Origin3d::zero(), extent: Extent3d::new_2d(width, height), array_layer: 0 }
    }

    pub fn mip(mip_level : u32, width : u32, height : u32) -> Self {
        Self { mip_level, origin: Origin3d::zero(), extent: Extent3d::new_2d(width, height), array_layer: 0 }
    }
}