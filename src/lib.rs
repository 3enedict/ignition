pub mod core;
pub mod ecs;
pub mod prelude;

use crate::core::options::Options;
use crate::core::rendering::{gpu::IgnitionGPU, window::IgnitionWindow};
use crate::ecs::IgnitionScene;

pub struct Engine {
    pub options: Options,

    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,

    pub scene: IgnitionScene,
}

impl Engine {
    pub fn ignite() -> Self {
        if env_logger::try_init().is_err() {
            println!("Warning: Unable to start env_logger");
        }

        pollster::block_on(Engine::setup_engine())
    }
}
