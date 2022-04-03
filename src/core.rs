pub mod rendering;
use rendering::{gpu::IgnitionGPU, window::IgnitionWindow};

pub mod shapes;
use shapes::Shape;

pub mod options;
use options::Options;

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
