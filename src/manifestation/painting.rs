use wgpu::{Texture, TextureDescriptor, TextureView};

use crate::{manifestation::Renderer, Engine};

impl<R: Renderer> Engine<R> {
    pub fn texture(&mut self, descriptor: &TextureDescriptor) -> (Texture, TextureView) {
        let texture = self.renderer.device().create_texture(&descriptor);
        let texture_view = texture.create_view(&Default::default());
        (texture, texture_view)
    }
}
