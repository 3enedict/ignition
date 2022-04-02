pub mod rendering;
use rendering::{gpu::IgnitionGPU, window::IgnitionWindow};

pub mod shapes;

pub mod options;
use options::Options;

pub struct Engine {
    pub options: Options,

    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,
}
impl Engine {
    pub fn ignite(options: Option<Options>) -> Self {
        if env_logger::try_init().is_err() {
            println!("Warning: Unable to start env_logger");
        }

        pollster::block_on(Engine::setup_engine(options.unwrap_or(Options::default())))
    }
}
