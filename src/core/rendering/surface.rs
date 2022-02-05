use std::sync::Arc;

use vulkano::swapchain::Surface;
use vulkano::instance::Instance;

use vulkano_win::VkSurfaceBuild;

use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub fn create_surface(
    instance: &Arc<Instance>,
    event_loop: &EventLoop<()>,
) -> Arc<Surface<Window>> {
    WindowBuilder::new()
        .build_vk_surface(event_loop, instance.clone())
        .unwrap()
}
