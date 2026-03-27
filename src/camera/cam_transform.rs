use crate::camera::{vec3_cross, vec3_dot, vec3_norm, vec3_sub};

///Represents the camera's transform at a 3D location.
#[derive(Clone, Debug)]
pub struct CameraTransform {
    pub position: [f32; 3],
    pub forward:  [f32; 3],
    pub up:       [f32; 3],
}

impl CameraTransform {
    pub fn new(position: [f32; 3], target: [f32; 3]) -> Self {
        Self {
            position,
            forward: vec3_norm(vec3_sub(target, position)),
            up:      [0.0, 1.0, 0.0],
        }
    }

    pub fn look_at(&mut self, target: [f32; 3], up: [f32; 3]) {
        self.forward = vec3_norm(vec3_sub(target, self.position));
        self.up      = vec3_norm(up);
    }
    ///Move and rotate the entire world so the camera is at the origin looking down a fixed axis.
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let f = vec3_norm(self.forward);
        let r = vec3_norm(vec3_cross(f, self.up));
        let u = vec3_cross(r, f);
        let p = self.position;
        [
            [r[0],  u[0], -f[0], 0.0],
            [r[1],  u[1], -f[1], 0.0],
            [r[2],  u[2], -f[2], 0.0],
            [-vec3_dot(r,p), -vec3_dot(u,p), vec3_dot(f,p), 1.0],
        ]
    }
}