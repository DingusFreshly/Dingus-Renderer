use std::sync::Arc;
use winit::window::{Window, WindowId};
use crate::context::RenderContext;
use crate::graph::RenderGraph;
use crate::commands::prelude::RenderQueue;
use crate::debug::{DebugStats, TimestampQuerySet};
use crate::registry::SurfaceRegistry;
use crate::registry::ResourceRegistry;

/// Top-level renderer resource. Insert into your ECS world once at startup.
pub struct Renderer {
    context:    Arc<RenderContext>,
    surfaces:   SurfaceRegistry,
    resources:  ResourceRegistry,
    graph:      RenderGraph,
    queue:      RenderQueue,
    staging:    StagingBelt,
    pool:       BufferPool,
    timestamps: Option<TimestampQuerySet>,
    debug:      DebugStats,
    frame_idx:  u64,
}