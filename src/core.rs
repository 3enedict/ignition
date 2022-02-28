use std::rc::Weak;
use std::cell::RefCell;

pub mod rendering;
use rendering::{
    window::IgnitionWindow,
    gpu::IgnitionGPU,
};

pub mod shapes;
use shapes::triangle::Triangle;

pub mod options;
use options::IgnitionOptions;


pub struct Engine {
    pub options: IgnitionOptions,

    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,

    pub shapes: Vec<Weak<RefCell<Triangle>>>,
}

impl Engine {
    pub fn ignite() -> Self {
        env_logger::init();

        pollster::block_on(Engine::setup_engine())
    }
}

