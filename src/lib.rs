pub mod core;
pub mod objects;
pub mod systems;

use vulkano::pipeline::graphics::viewport::Viewport;

use winit::event_loop::EventLoop;

use crate::core::parameters::VglRendererParameters;
use crate::core::surface::VglSurface;
use crate::core::logical_device::VglLogicalDevice;
use crate::core::swapchain::VglSwapchain;
use crate::core::render_pass::VglRenderPass;
use crate::core::pipeline::VglPipeline;
use crate::core::framebuffers::VglFramebuffers;
use crate::core::future::VglFuture;

use crate::objects::VglObject;



#[cfg(all(debug_assertions))]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;



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
}
