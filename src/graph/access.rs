use crate::handle::aliases::TextureHandle;
use crate::handle::Handle;

/// What a rander command does to a resource.
pub enum  AttachmentAccess {
    Read,
    Write,
    ReadWrite
}
/// A named reference to a texture used as input or output of a pass.
pub struct PassAttachment {
    pub name: &'static str,
    pub(crate) handle: TextureHandle,
    access: AttachmentAccess
}
impl PassAttachment {
    pub fn read(name: &'static str, handle: TextureHandle) -> Self { Self { name,handle, access: AttachmentAccess::Read } }
    pub fn write(name: &'static str, handle: TextureHandle) -> Self { Self { name,handle, access: AttachmentAccess::Write } }
    pub fn read_write(name: &'static str, handle: TextureHandle) -> Self { Self { name,handle, access: AttachmentAccess::ReadWrite } }
}