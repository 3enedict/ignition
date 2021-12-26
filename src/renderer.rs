use vulkano::pipeline::graphics::viewport::Viewport;

use winit::event_loop::EventLoop;

pub mod core;

use crate::renderer::core::parameters::VglRendererParameters;
use crate::renderer::core::surface::VglSurface;
use crate::renderer::core::logical_device::VglLogicalDevice;
use crate::renderer::core::swapchain::VglSwapchain;
use crate::renderer::core::render_pass::VglRenderPass;
use crate::renderer::core::pipeline::VglPipeline;
use crate::renderer::core::framebuffers::VglFramebuffers;
use crate::renderer::core::future::VglFuture;


use crate::objects::triangle::VglTriangle;
use crate::objects::rectangle::VglRectangle;
use crate::objects::vertex::Vertex;


pub struct VglRenderer {
    parameters: VglRendererParameters,

    event_loop: Option<EventLoop<()>>,
    surface: VglSurface,

    logical_device: VglLogicalDevice,

    swapchain: VglSwapchain,

    rectangles: VglRectangle,

    render_pass: VglRenderPass,

    pipeline: VglPipeline,

    viewport: Viewport,
    framebuffers: VglFramebuffers,

    future: VglFuture,

    recreate_swapchain: bool,
}

impl VglRenderer {
    pub fn logical_device(&self) -> &VglLogicalDevice {
        &self.logical_device
    }

    pub fn add_rectangles(
        &mut self,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
    ) {
        self.rectangles.add(vertices, indices);

        self.rectangles.generate_buffers(&self.logical_device);
    }

    pub fn add_system_setup(
        mut self,
        setup: fn(&mut VglRenderer),
    ) -> Self {
        setup(&mut self);

        self
        }
}
