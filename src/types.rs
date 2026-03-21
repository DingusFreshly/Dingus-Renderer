use wgpu;
///A 3D integer coordinate used as an offset into a texture or buffer region.
/// TODO! mark use cases for structs in this file
use std::convert::From;
#[derive(Debug, Clone, Copy)]
pub struct Origin3d {
    x: u32,
    y: u32,
    z: u32
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
    fn from(o: Origin3d) -> Self {
        wgpu::Origin3d {
            x: o.x,
            y: o.y,
            z: o.z
        }    
    }
}

///A 3d size.`depth_or_array_layers` is 1 for 2d textures, and N otherwise
#[derive(Debug, Clone, Copy)]
pub struct Extent3d {
    width: u32,
    height: u32,
    depth_or_array_layers: u32
}
impl Extent3d {
    pub fn new(width : u32, height : u32, depth_or_array_layers : u32) -> Extent3d {
        Self {
            width, height, depth_or_array_layers
        }
    }
    pub fn new_2d(width: u32, height: u32) ->Extent3d {
        Self {
            width,height,depth_or_array_layers: 1
        }

    }
    pub fn to_wgpu(self) -> wgpu::Extent3d {
        wgpu::Extent3d {
            height: self.height,
            width: self.width,
            depth_or_array_layers: self.depth_or_array_layers
        }
    }
}
impl From<Extent3d> for wgpu::Extent3d {
    fn from(o: Extent3d) -> wgpu::Extent3d {
        wgpu::Extent3d {
            height: o.height,
            width: o.width,
            depth_or_array_layers: o.depth_or_array_layers
        }
    }
}

///An integer 2d rectangle, used for scissor tests and copy regions
/// *Scissor test*: only render to square cutout of screen, used for viewports
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32
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
///Floating point viewport with near and far depth range. Used to limit rendering to a sub region of the screen
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    x: f32,
    y: f32,

    width: f32,
    height: f32,

    ///when viewport can start seeing, and when it stops
    min_depth: f32,
    max_depth: f32
}

impl Viewport {
    /// a viewport covering the full surface with depth range 0.0 - 1.0
    pub fn full(width : f32, height : f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            
            width,
            height,
            
            min_depth : 0.0,
            max_depth : 1.0,
        }
    }
}

//imageDataLayout
//ImageRegion

