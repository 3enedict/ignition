use wgpu::RenderPipeline;

pub mod rendering;
use rendering::{
    window::IgnitionWindow,
    gpu::IgnitionGPU,
};


pub struct Engine {
    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,
    pub pipelines: Vec<RenderPipeline>,
}

impl Engine {
    pub fn ignite() -> Self {
        pollster::block_on(Engine::setup_engine())
    }
}

