pub mod rendering;
pub mod objects;

use std::sync::Arc;

use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::device::{Device, Queue};
use vulkano::swapchain::{Swapchain, Surface};
use vulkano::render_pass::{Framebuffer, RenderPass};
use vulkano::sync::GpuFuture;

use winit::window::Window;
use winit::event_loop::EventLoop;

use crate::core::objects::VulkanObject;

pub struct VglRenderer {
    pub event_loop: Option<EventLoop<()>>,
    surface: Arc<Surface<Window>>,

    logical_device: Arc<Device>,
    queue: Arc<Queue>,

    swapchain: Arc<Swapchain<Window>>,

    objects: Vec<VulkanObject>,

    render_pass: Arc<RenderPass>,

    pipelines: Vec<Arc<GraphicsPipeline>>,

    viewport: Viewport,
    framebuffers: Vec<Arc<Framebuffer>>,

    future: Option<Box<dyn GpuFuture>>,

    pub recreate_swapchain: bool,
}
