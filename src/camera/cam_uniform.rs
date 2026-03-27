use bytemuck::{Pod, Zeroable};
use crate::camera::mat4_identity;

/// Uploaded to a uniform buffer every frame. All matrices are column-major
/// to match WGSL convention. Struct is 256 bytes, 16-byte aligned.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    /// Transformation of the camera from the origin, alligned with the Z axis.
    pub view:          [[f32; 4]; 4],
    /// Maps 3D world points in homogeneous coordinates to 2D image points in homogeneous coordinates.
    pub proj:          [[f32; 4]; 4],
    /// Combine view and projection to apply once
    pub view_proj:     [[f32; 4]; 4],
    pub inv_view:      [[f32; 4]; 4],
    pub inv_proj:      [[f32; 4]; 4],
    /// clip → world; used in raymarching / non-euclidean fragment shaders.
    pub inv_view_proj: [[f32; 4]; 4],
    pub position:      [f32; 3],
    ///Guarentees the struct matches the formatting and padding requirements
    pub _pad0:         f32,
    pub near:          f32,
    pub far:           f32,
    pub fov_y_radians: f32,
    pub aspect:        f32,
    pub width:         f32,
    pub height:        f32,
    pub _pad1:         [f32; 2],
}

impl CameraUniform {
    pub fn identity() -> Self {
        let id = mat4_identity();
        Self {
            view: id, proj: id, view_proj: id,
            inv_view: id, inv_proj: id, inv_view_proj: id,
            position: [0.0; 3], _pad0: 0.0,
            near: 0.1, far: 1000.0,
            fov_y_radians: std::f32::consts::FRAC_PI_3,
            aspect: 1.0, width: 1.0, height: 1.0, _pad1: [0.0; 2],
        }
    }
}