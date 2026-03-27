
/// Matrix that maps 3D world points in homogeneous coordinates to 2D image points in a pinhole camera model
/// https://www.youtube.com/watch?v=U0_ONQQ5ZNM
#[derive(Clone, Debug)]
pub enum CameraProjection {
    /// Portrays depth, use for 3D.
    Perspective  {
        fov_y: f32,
        ///Aspect ratio of screen
        aspect: f32,
        near: f32, far: f32
    },
    /// Flat, use for 2D scenes
    Orthographic { left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32 },
    /// Caller supplies an arbitrary projection matrix (hyperbolic, spherical, fisheye, etc.).
    Custom       { matrix: [[f32; 4]; 4] },
}
impl CameraProjection {
    pub fn projection_matrix(&self) -> [[f32; 4]; 4] {
        match self {
            CameraProjection::Perspective { fov_y, aspect, near, far } => {
                let f = 1.0 / (fov_y / 2.0).tan();
                let r = far - near;
                [
                    [f / aspect, 0.0,  0.0,                      0.0],
                    [0.0,        f,    0.0,                      0.0],
                    [0.0,        0.0,  -(far + near) / r,        -1.0],
                    [0.0,        0.0,  -2.0 * far * near / r,    0.0],
                ]
            }
            CameraProjection::Orthographic { left, right, bottom, top, near, far } => {
                let size_x = right - left;
                let size_y = top   - bottom;
                let size_z = far   - near;
                //we divide 2 by the sizes here because we need it to fit between [-1, 1]
                [
                    [2.0/size_x, 0.0,     0.0,      0.0],
                    [0.0,     2.0/size_y, 0.0,      0.0],
                    [0.0,     0.0,    -2.0/size_z,  0.0],
                    //moves the box so it's center is 0
                    [-(right+left)/size_x, -(top+bottom)/size_y, -(far+near)/size_z, 1.0],
                ]
            }
            CameraProjection::Custom { matrix } => *matrix,
        }
    }

    pub fn set_aspect(&mut self, new_aspect: f32) {
        if let CameraProjection::Perspective { aspect, .. } = self {
            *aspect = new_aspect;
        }
    }

    pub fn near_far(&self) -> (f32, f32) {
        match self {
            CameraProjection::Perspective  { near, far, .. } => (*near, *far),
            CameraProjection::Orthographic { near, far, .. } => (*near, *far),
            CameraProjection::Custom { .. }                  => (0.1, 1000.0),
        }
    }

    pub fn fov_y(&self) -> f32 {
        if let CameraProjection::Perspective { fov_y, .. } = self { *fov_y } else { 0.0 }
    }

    pub fn aspect(&self) -> f32 {
        if let CameraProjection::Perspective { aspect, .. } = self { *aspect } else { 1.0 }
    }
}