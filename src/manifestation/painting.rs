use wgpu::{Texture, TextureDescriptor, TextureView};

use crate::Engine;

impl Engine {
    pub fn texture(&mut self, descriptor: &TextureDescriptor) -> (Texture, TextureView) {
        let texture = self.renderer.device.create_texture(&descriptor);
        let texture_view = texture.create_view(&Default::default());
        (texture, texture_view)
    }
}
