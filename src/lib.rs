use crate::{liberty::EngineBuilder, manifestation::Renderer};

pub mod liberty;
pub mod manifestation;
pub mod prelude;

pub fn logger() {
    if env_logger::try_init().is_err() {
        println!("Warning: Unable to start logger. This may be because it has already been started, especially during tests.");
    }
}

pub struct Engine {
    pub renderer: Renderer,
}

impl Engine {
    pub fn ignite() -> Self {
        logger();

        Self {
            renderer: Renderer::new(),
        }
    }

    pub fn parameters() -> EngineBuilder {
        EngineBuilder::new()
    }
}
