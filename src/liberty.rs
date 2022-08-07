use winit::window::WindowBuilder;

use crate::{logger, manifestation::Renderer, Engine};

pub mod window;

pub struct EngineBuilder {
    pub window: WindowBuilder,
}

impl EngineBuilder {
    pub fn default() -> Self {
        logger();

        Self {
            window: WindowBuilder::new(),
        }
    }

    pub fn ignite(self) -> Engine {
        Engine {
            renderer: Renderer::new(&self),
        }
    }
}
