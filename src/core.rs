pub mod rendering;
use rendering::{
    window::IgnitionWindow,
    gpu::IgnitionGPU,
};

pub mod shapes;
use shapes::IgnitionShapes;


pub struct Engine {
    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,

    pub shapes: IgnitionShapes,
}

impl Engine {
    pub fn ignite() -> Self {
        pollster::block_on(Engine::setup_engine())
    }
}

