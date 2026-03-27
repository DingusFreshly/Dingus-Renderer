use std::collections::HashMap;
use winit::window::WindowId;
use crate::context::{RenderContext, SurfaceContext};
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

    pub fn add(&mut self, id: WindowId, ctx: SurfaceContext, sample_count: u32) -> Result<SurfaceContext, RendererError> {
        self.surfaces.insert(id, ctx).ok_or_else(|| { RendererError::InvalidWindow(id) })
    }

    pub fn remove(&mut self, id : WindowId) -> Result<SurfaceContext, RendererError> {
        self.surfaces.remove(&id).ok_or_else(|| {RendererError::InvalidWindow(id)})
    }

    pub fn resize(&mut self, id: WindowId, ctx: &RenderContext, w: u32, h: u32) -> Result<(), RendererError> {
        if let Some(s) = self.surfaces.get_mut(&id) { s.resize(ctx, w, h); }
        else {
            return Err(RendererError::InvalidWindow(id));
        }
        Ok(())
    }
    pub fn get(&self, id: WindowId)         -> Option<&SurfaceContext>    { self.surfaces.get(&id) }
    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut SurfaceContext>{ self.surfaces.get_mut(&id) }
    pub fn contains(&self, id: WindowId)    -> bool                       { self.surfaces.contains_key(&id) }
    pub fn iter_window_ids(&self)                -> impl Iterator<Item = WindowId> + '_ { self.surfaces.keys().copied() }
}