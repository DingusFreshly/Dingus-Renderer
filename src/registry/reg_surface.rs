use std::collections::HashMap;
use winit::window::WindowId;
use crate::context::prelude::SurfaceContext;
use crate::error::RendererError;

///Stores all the surfaces in the app, maps window id to surface
pub struct SurfaceRegistry {
    surfaces : HashMap<WindowId, SurfaceContext>,
}

impl SurfaceRegistry {
    pub fn new()-> Self {
        Self {
            surfaces: HashMap::new()
        }
    }

    pub fn add(&mut self, id: WindowId, ctx: SurfaceContext, sample_count: u32) -> Result<(), RendererError> {
        self.surfaces.insert(id, ctx).ok_or_else(|| {
            RendererError::InvalidWindow(id)
        });

        Ok(())
    }

    pub fn remove(&mut self, id : WindowId) -> Result<SurfaceContext, RendererError> {
        let surf = self.surfaces.remove(&id).ok_or_else(|| {RendererError::InvalidWindow(id)}).unwrap();
        Ok(surf)
    }
}