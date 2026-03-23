mod res_texture;
mod res_buffer;
mod res_pipeline;
mod res_mesh;
mod res_sampler;
mod res_shader;
mod res_registry;

pub mod prelude {
    use super::*;
    pub use res_buffer::*;
    pub use res_registry::*;
    pub use res_sampler::*;
    pub use res_shader::*;
    pub use res_mesh::*;
    pub use res_pipeline::*;
    pub use res_texture::*;
}
