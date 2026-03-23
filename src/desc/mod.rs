mod desc_texture;
mod desc_buffer;
mod desc_vertex;
mod desc_shader;
mod desc_pipeline;
mod desc_sampler;
mod desc_bind_group;

pub mod prelude {
    pub use crate::desc::{
        desc_texture::*,
        desc_buffer::*,
        desc_vertex::*,
        desc_shader::*,
        desc_pipeline::*,
        desc_sampler::*,
        desc_bind_group::*,
    };
}



