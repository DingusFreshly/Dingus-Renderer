use std::sync::Arc;
use crate::context::RenderContext;
use crate::graph::RenderGraph;
use crate::commands::prelude::RenderQueue;
use crate::debug::{DebugStats, TimestampQuerySet};
use crate::desc::prelude::BufferDesc;
use crate::handle::aliases::BufferHandle;
use crate::memory::{BufferPool, StagingBelt};
use crate::registry::SurfaceRegistry;
use crate::registry::ResourceRegistry;
use crate::resources::prelude::GpuBuffer;

/// Top-level renderer resource. Insert into your ECS world once at startup.
pub struct Renderer<'a> {
    context:    Arc<RenderContext>,
    surfaces:   SurfaceRegistry,
    resources:  ResourceRegistry,
    graph:      RenderGraph,
    queue:      RenderQueue,
    staging:    StagingBelt<'a>,
    pool:       BufferPool,
    timestamps: Option<TimestampQuerySet>,
    debug:      DebugStats,
    frame_idx:  u64,
}

impl<'a> Renderer<'a> {
    pub fn create_buffer(&mut self, desc: BufferDesc) -> BufferHandle {
        let b = GpuBuffer::create(&self.context.device, desc);
        self.resources.insert_buffer(b)
    }
}