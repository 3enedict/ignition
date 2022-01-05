use std::sync::Arc;

use vulkano::swapchain::Surface;

use vulkano_win::VkSurfaceBuild;

use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use crate::core::VglInstance;

pub struct VglSurface {
    surface: Arc<Surface<Window>>,
}

impl VglSurface {
    pub fn new(
        instance: &VglInstance,
        event_loop: &EventLoop<()>,
    ) -> Self {
        let surface = WindowBuilder::new()
            .build_vk_surface(event_loop, instance.clone_instance())
            .unwrap();

        Self {
            surface,
        }
    }

    pub fn get_surface(&self) -> &Arc<Surface<Window>> {
        &self.surface
    }

    pub fn clone_surface(&self) -> Arc<Surface<Window>> {
        self.surface.clone()
    }
}
