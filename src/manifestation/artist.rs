use winit::dpi::PhysicalSize;

use crate::{manifestation::lift_off::screen::Screen, Engine};

pub mod commands;
pub mod pipeline;

impl Engine<Screen> {
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.size = new_size;

            self.renderer.config.width = new_size.width;
            self.renderer.config.height = new_size.height;

            self.configure_surface();
        }
    }

    pub fn configure_surface(&mut self) {
        self.renderer
            .surface
            .configure(&self.renderer.gpu.device, &self.renderer.config);
    }
}
