use wgpu::{
    Backends, DeviceDescriptor, Features, Limits, PowerPreference, PresentMode, TextureUsages,
};
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

pub struct Configuration<'a> {
    pub title: &'static str,

    pub backend: Backends,
    pub device_options: DeviceDescriptor<'a>,
    pub power_preference: PowerPreference,
    pub force_fallback_adapter: bool,
    pub texture_usages: TextureUsages,
    pub present_mode: PresentMode,

    pub runtime_config: RuntimeConfiguration,
}

impl Default for Configuration<'_> {
    fn default() -> Self {
        Self {
            title: "Darkweb",

            backend: Backends::all(),
            device_options: DeviceDescriptor::default(),
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            texture_usages: TextureUsages::RENDER_ATTACHMENT,
            present_mode: PresentMode::Fifo,

            runtime_config: RuntimeConfiguration::default(),
        }
    }
}

impl Configuration<'_> {
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

    pub fn features(mut self, features: Features) -> Self {
        self.device_options.features = features;
        self
    }

    pub fn limits(mut self, limits: Limits) -> Self {
        self.device_options.limits = limits;
        self
    }

    pub fn power_preference(mut self, power_preference: PowerPreference) -> Self {
        self.power_preference = power_preference;
        self
    }

    pub fn force_fallback_adapter(mut self) -> Self {
        self.force_fallback_adapter = true;
        self
    }

    pub fn texture_usages(mut self, texture_usages: TextureUsages) -> Self {
        self.texture_usages = texture_usages;
        self
    }

    pub fn present_mode(mut self, present_mode: PresentMode) -> Self {
        self.present_mode = present_mode;
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
