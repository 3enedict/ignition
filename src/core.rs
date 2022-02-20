use wgpu::RenderPipeline;

pub mod rendering;
use rendering::window::IgnitionWindow;
use rendering::gpu::IgnitionGPU;

pub struct Engine {
    pub window: IgnitionWindow,
    pub gpu: IgnitionGPU,
    pub render_pipeline: RenderPipeline,
}

impl Engine {
    pub fn ignite() -> Self {
        pollster::block_on(Engine::setup_engine())
    }
}

