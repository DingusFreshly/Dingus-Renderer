use wgpu::*;

#[derive(Debug, Clone, Copy)]
struct Origin3d {

    x : u32,
    y : u32,
    z : u32,

}

impl Origin3d {

    pub fn new(x : u32, y : u32, z : u32) -> Self {
        Self { x, y, z }
    }
    
    pub fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
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

#[derive(Debug, Clone, Copy)]
struct Extent3d {

    width : u32,
    height : u32,
    depth : u32,

}

impl Extent3d {

    pub fn new(width : u32, height : u32, depth : u32) -> Self {
        Self { width, height, depth }
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

#[derive(Debug, Clone, Copy)]
struct Rect {

    x : u32,
    y : u32,
    width : u32,
    height : u32,

}

impl Rect {

    pub fn new(x : u32, y : u32, width : u32, height : u32) -> Self {
        Self { x, y, width, height }
    }

    pub fn full(width : u32, height : u32) -> Self {
        Self { x: 0, y: 0, width, height }
    }

}

#[derive(Debug, Clone, Copy)]
struct Viewport {

    x : f32,
    y : f32,
    width : f32,
    height : f32,
    min_depth : f32,
    max_depth : f32,

}

impl Viewport {

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

    pub fn new_2d(width : u64, bytes_per_pixel : Option<u32>) -> Self {
        Self { offset: width, bytes_per_row: None, rows_per_image: None }
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