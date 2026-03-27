use wgpu::{CommandEncoder, Device, Queue};
use crate::commands::prelude::RenderCommand;
use crate::context::SurfaceContext;
use crate::registry::res_registry::ResourceRegistry;

/// Given to each pass during execution. Includes everything needed to record GPU commands.
pub struct PassContext<'a> {
    pub(crate) encoder: &'a mut CommandEncoder,
    pub(crate) resources: &'a ResourceRegistry,
    pub(crate) surface: &'a SurfaceContext,
    /// Draw/compute commands sorted by sort_key, filtered for this pass.
    pub(crate) commands: &'a [RenderCommand],
    /// Monotonically increasing frame counter. `frame_idx % 2` gives ping-pong index.
    pub frame_idx: u64,
    pub device:    &'a Device,
    pub queue:     &'a Queue,
}