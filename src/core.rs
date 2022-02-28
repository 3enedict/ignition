pub mod rendering;
use rendering::{
    window::IgnitionWindow,
    gpu::IgnitionGPU,
};

pub mod shapes;

pub mod options;
use options::IgnitionOptions;


pub struct Engine {
    pub options: IgnitionOptions,

    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,
}

impl Engine {
    pub fn ignite() -> Self {
        env_logger::init();

        pollster::block_on(Engine::setup_engine())
    }
}

