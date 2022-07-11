#[macro_use]
extern crate derive_builder;

pub mod liberty;
pub mod life;
pub mod manifestation;
pub mod prelude;

use crate::liberty::{Parameters, ParametersBuilder};
use crate::life::Scene;
use crate::manifestation::Renderer;

pub struct Engine {
    pub renderer: Renderer,
    pub scene: Scene,

    pub parameters: Parameters,
}

impl Engine {
    pub fn ignite() -> Self {
        Self::env_logger();
        Engine::setup_engine(ParametersBuilder::default().build().unwrap())
    }

    pub fn parameters() -> ParametersBuilder {
        ParametersBuilder::default()
    }

    pub fn env_logger() {
        if env_logger::try_init().is_err() {
            println!("Warning: Unable to start env_logger");
        }
    }

    pub fn setup_engine(parameters: Parameters) -> Engine {
        Self {
            renderer: Renderer::new(&parameters),
            scene: Scene::new(),

            parameters,
        }
    }
}
