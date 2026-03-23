use crate::desc::prelude::SamplerDesc;
///how shaders interpret texture data, look at `SamplerDesc` for the bulk of the properties.
#[derive(Clone)]
pub struct Sampler {
    pub sampler: wgpu::Sampler,
    pub desc:    SamplerDesc,
}

impl Sampler {
    pub fn create(device: &wgpu::Device, desc: SamplerDesc) -> Self {
        let sampler = device.create_sampler(&desc.to_wgpu());
        Self { sampler, desc }
    }
}