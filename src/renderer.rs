use std::sync::Arc;

use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::pipeline::viewport::Viewport;
use vulkano::sync::GpuFuture;

use winit::event_loop::EventLoop;

pub mod setup;
use crate::renderer::setup::Vertex;

use crate::renderer::setup::instance::VglInstance;
use crate::renderer::setup::surface::VglSurface;
use crate::renderer::setup::logical_device::VglLogicalDevice;
use crate::renderer::setup::swapchain::VglSwapchain;
use crate::renderer::setup::render_pass::VglRenderPass;
use crate::renderer::setup::pipeline::VglPipeline;
use crate::renderer::setup::framebuffers::VglFramebuffers;


pub mod runtime;

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
