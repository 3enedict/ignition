use wgpu::Backends;
use winit::{dpi::PhysicalSize, event_loop::ControlFlow};

use crate::{
    manifestation::lift_off::{headless::Headless, image::Image, screen::Screen},
    Engine,
};

pub struct RuntimeConfiguration {
    pub control_flow: ControlFlow,
    pub any_thread: bool,

    pub size: PhysicalSize<u32>,
}

impl Default for RuntimeConfiguration {
    fn default() -> Self {
        Self {
            control_flow: ControlFlow::Poll,
            any_thread: false,

            size: PhysicalSize {
                width: 1920,
                height: 1080,
            },
        }
    }
}

pub struct Configuration {
    pub title: &'static str,
    pub backend: Backends,

    pub runtime_config: RuntimeConfiguration,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            title: "Darkweb",
            backend: Backends::all(),

            runtime_config: RuntimeConfiguration::default(),
        }
    }
}

impl Configuration {
    pub fn ignite(self) -> Engine<Screen> {
        Engine::configuration(self)
    }

    pub fn headless(self) -> Engine<Headless> {
        Engine::configuration(self)
    }

    pub fn image(self) -> Engine<Image<'static>> {
        Engine::configuration(self)
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn backend(mut self, backend: Backends) -> Self {
        self.backend = backend;
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
