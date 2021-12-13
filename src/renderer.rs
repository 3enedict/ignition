use std::sync::Arc;

use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::pipeline::viewport::Viewport;
use vulkano::sync::GpuFuture;

use winit::event_loop::EventLoop;

pub mod core;
use crate::renderer::core::Vertex;

use crate::renderer::core::surface::VglSurface;
use crate::renderer::core::logical_device::VglLogicalDevice;
use crate::renderer::core::swapchain::VglSwapchain;
use crate::renderer::core::render_pass::VglRenderPass;
use crate::renderer::core::pipeline::VglPipeline;
use crate::renderer::core::framebuffers::VglFramebuffers;


pub struct VglRenderer {
    event_loop: EventLoop<()>,
    surface: VglSurface,

    logical_device: VglLogicalDevice,

    swapchain: VglSwapchain,

    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,

    render_pass: VglRenderPass,

    pipeline: VglPipeline,

    viewport: Viewport,
    framebuffers: VglFramebuffers,

    previous_frame_end: Option<Box<dyn GpuFuture>>,

    recreate_swapchain: bool,
}
