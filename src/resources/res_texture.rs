/// A texture on the gpu that lives on the swapchain
pub struct TextureRes {
    ///the raw GPU texture
    texture: wgpu::Texture,
    ///the default view (all mips, all layers). Used for most shader bindings.
    view: wgpu::TextureView,
    ///the original descriptor, kept for resize logic and tooling
    //TODO!desc: TextureDesc,
    ///approximate GPU memory usage, tracked for debug stats
    size_bytes: u64
}