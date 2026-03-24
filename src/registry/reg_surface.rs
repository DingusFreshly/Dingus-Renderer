use std::collections::HashMap;
use winit::window::WindowId;
use crate::context::prelude::SurfaceContext;

///Stores all the surfaces in the app, maps window id to surface
pub struct SurfaceRegistry {
    surfaces : HashMap<WindowId, SurfaceContext>,
}