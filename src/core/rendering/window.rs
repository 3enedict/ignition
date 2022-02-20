use wgpu::{
    Instance,
    Surface,
    Adapter,

    SurfaceConfiguration,
    TextureUsages,
    PresentMode,
};

use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
};

use crate::core::Engine;

pub struct IgnitionWindow {
    pub event_loop: Option<EventLoop<()>>, 

    pub window: Window, 
    pub size: PhysicalSize<u32>,

    pub surface: Surface, 
    pub config: SurfaceConfiguration,
}

pub fn create_window() -> (EventLoop<()>, Window, PhysicalSize<u32>) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();

    (event_loop, window, size)
}

pub fn create_surface(instance: &Instance, window: &Window) -> Surface {
    unsafe { instance.create_surface(&window) }
}

pub fn generate_default_configuration(size: &PhysicalSize<u32>, surface: &Surface, adapter: &Adapter) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(adapter).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
    }
}

impl Engine {
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.window.size = new_size;

            self.window.config.width = new_size.width;
            self.window.config.height = new_size.height;

            self.window.surface.configure(&self.gpu.device, &self.window.config);
        }
    }
}
