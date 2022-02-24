pub mod rendering;
use rendering::{
    window::IgnitionWindow,
    gpu::IgnitionGPU,
};

pub mod shapes;
use shapes::IgnitionShapes;

pub mod options;
use options::IgnitionOptions;


pub struct Engine {
    pub options: IgnitionOptions,

    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,

    pub shapes: IgnitionShapes,
}

impl Engine {
    pub fn ignite(options: IgnitionOptions) -> Self {
        env_logger::init();

        pollster::block_on(Engine::setup_engine(options))
    }
}

