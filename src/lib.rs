extern crate component_derive;

use crate::{
    liberty::{Configuration, RuntimeConfiguration},
    manifestation::{
        lift_off::{headless::Headless, image::Image, screen::Screen},
        Renderer,
    },
};

pub mod liberty;
pub mod life;
pub mod manifestation;

pub mod prelude {
    pub use component_derive::Component;
    pub use wgpu::include_wgsl;

    pub use crate::{
        liberty::Configuration,
        life::Component,
        manifestation::{
            lift_off::{headless::Headless, image::Image, screen::Screen},
            Renderer,
        },
        Engine,
    };
}

pub struct Engine<R: Renderer> {
    pub renderer: R,

    pub config: RuntimeConfiguration,
}

impl Engine<Screen> {
    pub fn ignite() -> Self {
        Self::configuration(Configuration::default())
    }
}

impl Engine<Headless> {
    pub fn headless() -> Self {
        Self::configuration(Configuration::default())
    }
}

impl Engine<Image<'static>> {
    pub fn image() -> Self {
        Self::configuration(Configuration::default())
    }
}

impl<R: Renderer> Engine<R> {
    pub fn configuration(mut config: Configuration) -> Self {
        logger();

        Engine {
            renderer: R::new(&mut config),

            config: config.runtime_config,
        }
    }
}

pub fn logger() {
    if env_logger::try_init().is_err() {
        println!("Warning: Unable to start logger (this may be because it has already been started, especially during tests) - Ignition");
    }
}
