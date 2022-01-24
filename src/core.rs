pub mod rendering;
pub mod objects;

use vulkano::pipeline::graphics::viewport::Viewport;

use winit::event_loop::EventLoop;

use crate::core::rendering::parameters::VglRendererParameters;
use crate::core::rendering::surface::VglSurface;
use crate::core::rendering::logical_device::VglLogicalDevice;
use crate::core::rendering::swapchain::VglSwapchain;
use crate::core::rendering::render_pass::VglRenderPass;
use crate::core::rendering::pipeline::VglPipeline;
use crate::core::rendering::framebuffers::VglFramebuffers;
use crate::core::rendering::future::VglFuture;

use crate::core::objects::VulkanObject;

pub trait Base {
    fn run(self);

    fn get_renderer(&mut self) -> &mut VglRenderer;
}

pub struct VglRenderer {
    parameters: VglRendererParameters,

    event_loop: Option<EventLoop<()>>,
    surface: VglSurface,

    logical_device: VglLogicalDevice,

    swapchain: VglSwapchain,

    objects: Vec<VulkanObject>,

    render_pass: VglRenderPass,

    pipelines: Vec<VglPipeline>,

    viewport: Viewport,
    framebuffers: VglFramebuffers,

    future: VglFuture,

    recreate_swapchain: bool,
}
