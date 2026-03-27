use crate::camera::{mat4_inverse, mat4_mul, CameraProjection, CameraTransform, CameraUniform};
use crate::commands::prelude::RenderQueue;
use crate::desc::prelude::BufferDesc;
use crate::handle::aliases::{BindGroupHandle, BufferHandle};
/// Describes and holds handles to the full cameras translation, fov, projection, and it's data gpu.
pub struct Camera {
    pub projection:     CameraProjection,
    pub transform:      CameraTransform,
    uniform:            CameraUniform,
    pub gpu_buffer:     BufferHandle,
    pub gpu_bind_group: BindGroupHandle,
}
impl Camera {
    /// Allocate the GPU uniform buffer. Call once at startup.
    pub fn new(
        projection: CameraProjection,
        transform:  CameraTransform,
        renderer:   &mut crate::renderer::Renderer,
    ) -> Self {
        let buf = renderer.create_buffer(BufferDesc {
            label:              Some("camera_uniform"),
            size:               std::mem::size_of::<CameraUniform>() as u64,
            usage:              wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut cam = Self {
            projection, transform,
            uniform:        CameraUniform::identity(),
            gpu_buffer:     buf,
            gpu_bind_group: BindGroupHandle::null(),
        };
        cam.update();
        cam
    }
    /// Recompute all matrices. Call once per frame before submitting draw calls.
    pub fn update(&mut self) {
        let view = self.transform.view_matrix();
        let proj = self.projection.projection_matrix();
        let vp   = mat4_mul(proj, view);
        let (near, far) = self.projection.near_far();

        self.uniform.view          = view;
        self.uniform.proj          = proj;
        self.uniform.view_proj     = vp;
        self.uniform.inv_view      = mat4_inverse(view);
        self.uniform.inv_proj      = mat4_inverse(proj);
        self.uniform.inv_view_proj = mat4_inverse(vp);
        self.uniform.position      = self.transform.position;
        self.uniform.near          = near;
        self.uniform.far           = far;
        self.uniform.fov_y_radians = self.projection.fov_y();
        self.uniform.aspect        = self.projection.aspect();
    }
    /// Push a WriteBuffer command to upload the current uniform to the GPU.
    pub fn write_to_queue(&self, queue: &mut RenderQueue) {
        queue.write_buffer(self.gpu_buffer, 0, bytemuck::bytes_of(&self.uniform).to_vec());
    }
    pub fn bind_group(&self) -> BindGroupHandle { self.gpu_bind_group }
    pub fn buffer(&self)     -> BufferHandle    { self.gpu_buffer }

    /// Update aspect and pixel dimensions, Call when the window is resized.
    pub fn set_surface_size(&mut self, w: u32, h: u32) {
        self.projection.set_aspect(w as f32 / h.max(1) as f32);
        self.uniform.width  = w as f32;
        self.uniform.height = h as f32;
    }
}