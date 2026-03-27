use std::vec::Drain;
use crate::commands::render_command::{DrawCall, RenderCommand};
use crate::desc::prelude::TextureDesc;
use crate::handle::aliases::{BufferHandle, TextureHandle};
use crate::types::Rect;

pub struct RenderQueue {
    commands: Vec<RenderCommand>
}
const INITIAL_CAPACITY : usize = 1024;
impl RenderQueue {
    ///New renderqueue with capacity 1024
    pub fn new() -> Self { Self{commands: Vec::with_capacity(INITIAL_CAPACITY)} }

   pub fn push(&mut self, cmd : RenderCommand) { self.commands.push(cmd); }
    pub fn push_batch(&mut self, cmd : impl IntoIterator<Item= RenderCommand>) { self.commands.extend(cmd); }

    pub fn draw(&mut self, call: DrawCall) { self.commands.push(RenderCommand::Draw(call))    }

    pub fn write_texture(&mut self, handle: TextureHandle, data: Vec<u8>, layout: TextureDesc, region: Rect) {
        self.commands.push(RenderCommand::WriteTexture {
            handle, data, layout, region
        })
    }
    pub fn write_buffer(&mut self, handle: BufferHandle, offset: u64, data: Vec<u8>) {
        self.commands.push(RenderCommand::WriteBuffer {
            handle, offset, data
        })
    }
    pub fn len(&self) -> usize {self.commands.len()}
    pub fn is_empty(&self) ->bool {self.commands.is_empty()}

    pub fn drain(&mut self) -> Drain<'_, RenderCommand> { self.commands.drain(0..self.len()) }
    pub fn clear(&mut self) {self.commands.clear()}
}