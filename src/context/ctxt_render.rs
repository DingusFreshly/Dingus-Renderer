use std::sync::Arc;
use crate::error::RendererError;

/// The GPU device and queue, shared via Arc across all renderer subsystems.
pub struct RenderContext {
    pub device:   wgpu::Device,
    pub queue:    wgpu::Queue,
    pub adapter:  wgpu::Adapter,
    pub features: wgpu::Features,
    pub limits:   wgpu::Limits,
    pub backend:  wgpu::Backend,
}

impl RenderContext {
    /// Initialize adapter, device, and queue, async because adapter selection
    /// involves asking the OS driver which GPU is available
    pub async fn new(backends: wgpu::Backends) -> Result<Arc<Self>, RendererError> {
        //TODO! figure out what the fuck the other parameters do
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::HighPerformance,
                compatible_surface:     None,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|x| RendererError::NoAdapter(backends))?;

        let features = adapter.features();
        let limits = adapter.limits();
        let backend = adapter.get_info().backend;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    //TODO! label properly
                    label:             Some("renderer_device"),
                    required_features: features,
                    required_limits:   limits.clone(),
                    memory_hints:      wgpu::MemoryHints::Performance,
                    ..Default::default()
                },
            )
            .await?;

        log::info!("GPU: {:?} via {:?}", adapter.get_info().name, backend);
        
        Ok(Arc::new(Self { device, queue, adapter, features, limits, backend }))
    }

    pub fn supports(&self, f: wgpu::Features) -> bool { self.features.contains(f) }
    pub fn backend_name(&self) -> &'static str {
        match self.backend {
            wgpu::Backend::Vulkan       => "Vulkan",
            wgpu::Backend::Metal        => "Metal",
            wgpu::Backend::Dx12         => "DirectX 12",
            wgpu::Backend::Gl           => "OpenGL ES",
            wgpu::Backend::BrowserWebGpu => "WebGPU",
            _                           => "Unknown",
        }
    }
}