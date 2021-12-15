use std::sync::Arc;

use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::pipeline::viewport::Viewport;

use winit::event_loop::EventLoop;

pub mod core;

use crate::renderer::core::surface::VglSurface;
use crate::renderer::core::logical_device::VglLogicalDevice;
use crate::renderer::core::swapchain::VglSwapchain;
use crate::renderer::core::render_pass::VglRenderPass;
use crate::renderer::core::pipeline::VglPipeline;
use crate::renderer::core::framebuffers::VglFramebuffers;
use crate::renderer::core::future::VglFuture;

use crate::objects::vertex::Vertex;
use crate::objects::triangle::VglTriangle;


pub struct VglRenderer {
    event_loop: EventLoop<()>,
    surface: VglSurface,

    logical_device: VglLogicalDevice,

    swapchain: VglSwapchain,

    vertex_buffer: Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,

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

    pub fn add_triangle(
        &mut self,
        triangle: VglTriangle,
    ) {
        self.vertex_buffer = Some(triangle.get_vertex_buffer());
    }
}
