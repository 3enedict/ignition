pub mod rendering;
use rendering::{gpu::IgnitionGPU, window::IgnitionWindow};

pub mod shapes;

pub mod options;
use options::{Options, OptionsBuilder, OptionsBuilderError};

pub struct Engine {
    pub options: Options,

    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,
}
impl Engine {
    pub fn ignite(options: Result<Options, OptionsBuilderError>) -> Self {
        if env_logger::try_init().is_err() {
            println!("Warning: Unable to start env_logger");
        }

        if options.is_err() {
            println!("Warning: Supplied options are incorrect. Reverting to default configuration.")
        }

        let default_options = OptionsBuilder::default().build().unwrap();
        pollster::block_on(Engine::setup_engine(options.unwrap_or(default_options)))
    }
}
