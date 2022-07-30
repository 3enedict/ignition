#[macro_use]
extern crate derive_builder;

macro_rules! unwrap {
    ($expression:expr) => {
        match $expression {
            Ok(value) => value,
            Err(e) => {
                log::warn!("{}", e);
                return;
            }
        }
    };
}

macro_rules! unwrap_or {
    ($expression:expr, $return_value:expr) => {
        match $expression {
            Ok(value) => value,
            Err(e) => {
                log::warn!("{}", e);
                return $return_value;
            }
        }
    };
}

pub mod liberty;
pub mod life;
pub mod manifestation;
pub mod prelude;

use crate::liberty::{Parameters, ParametersBuilder};
use crate::life::Scene;
use crate::manifestation::Renderer;

pub fn logger() {
    if env_logger::try_init().is_ok() {
        println!("Warning: Unable to start logger. This may be because it has already been started, especially during tests.");
    }
}

pub struct Engine {
    pub renderer: Renderer,
    pub scene: Scene,

    pub parameters: Parameters,
}

impl Engine {
    pub fn ignite() -> Self {
        logger();

        Engine::setup_engine(ParametersBuilder::default().build().unwrap())
    }

    pub fn parameters() -> ParametersBuilder {
        ParametersBuilder::default()
    }

    pub fn setup_engine(parameters: Parameters) -> Engine {
        Self {
            renderer: Renderer::new(&parameters),
            scene: Scene::new(),

            parameters,
        }
    }
}
