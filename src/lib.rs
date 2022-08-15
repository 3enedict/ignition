use wgpu::Backends;
use winit::event_loop::ControlFlow;

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

    pub config: RuntimeConfiguration,
}

impl Engine {
    pub fn ignite() -> Self {
        Self::ignite_conf(Configuration::default())
    }

    pub fn ignite_conf(config: Configuration) -> Self {
        logger();

        Engine {
            renderer: Renderer::new(&config),

            config: config.runtime_config,
        }
    }
}

/* Engine configuration */

pub struct RuntimeConfiguration {
    pub control_flow: ControlFlow,
    pub any_thread: bool,
}

impl Default for RuntimeConfiguration {
    fn default() -> Self {
        Self {
            control_flow: ControlFlow::Poll,
            any_thread: false,
        }
    }
}

pub struct Configuration {
    title: &'static str,
    backend: Backends,
    headless: bool,

    runtime_config: RuntimeConfiguration,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            title: "Darkweb",
            backend: Backends::all(),
            headless: false,

            runtime_config: RuntimeConfiguration::default(),
        }
    }
}

impl Configuration {
    pub fn ignite(self) -> Engine {
        Engine::ignite_conf(self)
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn backend(mut self, backend: Backends) -> Self {
        self.backend = backend;
        self
    }

    pub fn headless(mut self) -> Self {
        self.headless = true;
        self
    }

    pub fn any_thread(mut self) -> Self {
        self.runtime_config.any_thread = true;
        self
    }

    pub fn control_flow(mut self, control_flow: ControlFlow) -> Self {
        self.runtime_config.control_flow = control_flow;
        self
    }
}
