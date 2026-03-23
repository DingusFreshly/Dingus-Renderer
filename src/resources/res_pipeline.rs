pub struct Pipeline {
    pub kind: PipelineKind,
    pub label: Option<&'static str>,
}

pub enum PipelineKind {
    Compute(wgpu::ComputePipeline),
    Render(wgpu::RenderPipeline)
}
impl Pipeline {
    pub fn as_render(&self)  -> Option<&wgpu::RenderPipeline>  { match &self.kind { PipelineKind::Render(p)  => Some(p), _ => None } }
    pub fn as_compute(&self) -> Option<&wgpu::ComputePipeline> { match &self.kind { PipelineKind::Compute(p) => Some(p), _ => None } }
    pub fn is_compute(&self) -> bool { matches!(self.kind, PipelineKind::Compute(_)) }
}