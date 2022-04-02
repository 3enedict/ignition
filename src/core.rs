pub mod rendering;
use rendering::{gpu::IgnitionGPU, window::IgnitionWindow};

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
        if env_logger::try_init().is_err() {
            println!("Warning: Unable to start env_logger");
        }

        pollster::block_on(Engine::setup_engine())
    }
}
