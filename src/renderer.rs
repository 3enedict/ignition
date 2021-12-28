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


use crate::object::VglObject;
use crate::object::vertex::Vertex;


pub struct VglRenderer {
    parameters: VglRendererParameters,

    event_loop: Option<EventLoop<()>>,
    surface: VglSurface,

    logical_device: VglLogicalDevice,

    swapchain: VglSwapchain,

    objects: Vec<VglObject>,

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

    pub fn add_triangles(
        &mut self,
        vertices: &mut Vec<Vertex>,
    ) {
        let triangle = VglObject::triangle(&self.logical_device, vertices);
        self.objects.push(triangle);
    }

    pub fn add_rectangles(
        &mut self,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
    ) {
        let rectangle = VglObject::rectangle(&self.logical_device, vertices, indices);
        self.objects.push(rectangle);
    }

    pub fn add_system_setup(
        mut self,
        setup: fn(&mut VglRenderer),
    ) -> Self {
        setup(&mut self);

        self
        }
}
