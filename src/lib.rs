use wgpu::Backends;

use crate::manifestation::Renderer;

pub mod manifestation;
pub mod prelude;

pub fn logger() {
    if env_logger::try_init().is_err() {
        println!("Warning: Unable to start logger (this may be because it has already been started, especially during tests) - Ignition");
    }
}

pub struct Engine {
    pub renderer: Renderer,
}

impl Engine {
    pub fn ignite() -> Self {
        Self::ignite_conf(Configuration::default())
    }

    pub fn ignite_conf(config: Configuration) -> Self {
        logger();

        Engine {
            renderer: Renderer::new(&config),
        }
    }
}

pub struct Configuration {
    title: &'static str,
    backend: Backends,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            title: "Darkweb",
            backend: Backends::all(),
        }
    }
}

impl Configuration {
    pub fn ignite(self) -> Engine {
        Engine::ignite_conf(self)
    }

    pub fn title(self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn backend(self, backend: Backends) -> Self {
        self.backend = backend;
        self
    }
}
