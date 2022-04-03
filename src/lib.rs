pub mod core;
pub mod prelude;

use crate::core::options::Options;
use crate::core::rendering::{gpu::IgnitionGPU, window::IgnitionWindow};
use crate::core::shapes::Shape;

pub struct Intermediate {
    pub shape: Shape,
    pub render: bool,
}

pub struct Engine {
    pub options: Options,

    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,

    pub shapes: Vec<Intermediate>,
}

impl Engine {
    pub fn ignite() -> Self {
        if env_logger::try_init().is_err() {
            println!("Warning: Unable to start env_logger");
        }

        pollster::block_on(Engine::setup_engine())
    }
}
