use thiserror::Error;
use wgpu::{Backend, Features, RequestDeviceError, SurfaceError};
use winit::window::WindowId;

///Everything that can go wrong with the renderer (hopefully)
#[derive( Error,Debug)]
pub enum RendererError {

    #[error("Surface error, swapchain failed to provide frame: {0}")]
    SurfaceError(#[from] SurfaceError),

    #[error("No GPU found matching backends: {0}, choose different backends.")]
    NoAdapter(Backend),

    #[error("GPU rejected device request : {0}, try requesting fewer features or limits.")]
    DeviceRequest(#[from] RequestDeviceError),

    #[error("Shader {label} failed to compile. Message: {message}")]
    ShaderCompilation {
        label: String,
        message: String
    },
    #[error("Invalid handle, either previously destroyed or never created.")]
    InvalidHandle,

    #[error("Invalid window with id : {0:?}!")]
    InvalidWindow(WindowId),

    //mesh size mismatch
    #[error("Render graph contains a cyclic reference starting at {0}")]
    RenderGraphCycle(String),

    #[error("One or more GPU features not available: {0}")]
    FeatureNotAvailable(Features),

    //BufferTooSmall{}
    #[error("Surface configuration failed in window : {0:?}")]
    SurfaceConfig(WindowId)
}
