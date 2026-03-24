use wgpu::{SurfaceCapabilities, SurfaceConfiguration};
use crate::resources::prelude::Texture;
use winit::window::{Window, WindowId};
use crate::context::ctxt_render::RenderContext;
use std::sync::Arc;
use crate::desc::prelude::TextureDesc;
use crate::error::RendererError;

/// holds all the context related to one window
pub struct SurfaceContext {
    ///A drawable plane on a window
    surface: wgpu::Surface<'static>,

    capabilities: SurfaceCapabilities,
    ///Settings for the surface
    config: wgpu::SurfaceConfiguration,
    ///A depth texture, higher pixel values represent further pixels
    depth: Texture,
    ///An optional setting to smooth out jagged edges, places transparent pixels around corners to smooth out
    msaa: Option<Texture>,
    ///Number of sample points for MSAA (more == more MSAA)
    sample_count: u32,

    width: u32,
    height: u32
}

impl SurfaceContext {
    pub fn new(
        ctx: &Arc<RenderContext>,
        window: Arc<Window>,
        sample_count: u32
    ) -> Result<Self, RendererError>{
        let instance = &ctx.clone().instance;

        let surface  = instance
            .create_surface(window.clone())
            .map_err(|_| RendererError::SurfaceConfig(window.id()))?;

        let capabilities = surface.get_capabilities(&ctx.adapter);

        //we are getting all the rgb texture capabilities
        //TODO! look into supporting other formats
        let format = capabilities.formats.iter()
            .find(|f|f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        let sz = window.inner_size();
        let width = sz.width;
        let height = sz.height;

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            height,
            width,
            //TODO! look into this
            present_mode: wgpu::PresentMode::AutoVsync,
            //first is usually preferred
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,

        };

        surface.configure(&ctx.device, &config);

        let depth = Self::create_depth(&ctx.device, width, height);
        let msaa  = (sample_count > 1)
            .then(|| Self::create_msaa(&ctx.device, width, height, sample_count, format));

        Ok(Self {
            surface,
            capabilities,
            config,
            msaa,
            depth,
            width,
            height,
            sample_count

        })
    }
    /// Empty depth texture
    fn create_depth(device: &wgpu::Device, w: u32, h: u32) -> Texture {
        Texture::create(device, TextureDesc {
            label:           Some("depth"),
            width: w, height: h, depth_or_layers: 1, mip_levels: 1,
            format:          wgpu::TextureFormat::Depth32Float,
            usage:           wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            dimension:       wgpu::TextureDimension::D2,
            sample_count:    1,
        })
    }
    ///Empty msaa texture
    fn create_msaa(device: &wgpu::Device, w: u32, h: u32, sc: u32, fmt: wgpu::TextureFormat) -> Texture {
        Texture::create(device, TextureDesc {
            label:           Some("msaa"),
            width: w, height: h, depth_or_layers: 1, mip_levels: 1,
            format:          fmt,
            usage:           wgpu::TextureUsages::RENDER_ATTACHMENT,
            dimension:       wgpu::TextureDimension::D2,
            sample_count:    sc,
        })
    }

    /// Rebuild the swapchain, depth buffer, and MSAA texture at a new size.
    /// Call in response to `WindowEvent::Resized`.
    pub fn resize(&mut self, ctx: &RenderContext, w: u32, h: u32) {
        let (w, h) = (w.max(1), h.max(1));
        self.config.width  = w;
        self.config.height = h;
        self.width  = w;
        self.height = h;
        self.surface.configure(&ctx.device, &self.config);
        self.depth = Self::create_depth(&ctx.device, w, h);
        if self.sample_count > 1 {
            self.msaa = Some(Self::create_msaa(&ctx.device, w, h, self.sample_count, self.config.format));
        }
    }
    /// Aquire the next swapchain image. Call once per frame before drawing
    pub fn acquire(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }
    pub fn present(frame: wgpu::SurfaceTexture) { frame.present(); }

    //-----Debug
    pub fn depth_view(&self)        -> &wgpu::TextureView              { &self.depth.view }
    pub fn msaa_view(&self)         -> Option<&wgpu::TextureView>      { self.msaa.as_ref().map(|m| &m.view) }
    pub fn format(&self)            -> wgpu::TextureFormat             { self.config.format }
    pub fn sample_count(&self)      -> u32                             { self.sample_count }
}