use wgpu::{Adapter, Instance, PresentMode, Surface, SurfaceConfiguration, TextureUsages};

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    platform::unix::EventLoopExtUnix,
    window::{Window as WinitWindow, WindowBuilder},
};

use crate::Engine;

pub struct Window {
    pub event_loop: Option<EventLoop<()>>,

    pub window: WinitWindow,
    pub size: PhysicalSize<u32>,

    pub surface: Surface,
    pub config: SurfaceConfiguration,
}

pub fn create_window() -> (EventLoop<()>, WinitWindow, PhysicalSize<u32>) {
    let event_loop = EventLoop::new_any_thread();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let size = window.inner_size();

    (event_loop, window, size)
}

pub fn create_surface(instance: &Instance, window: &WinitWindow) -> Surface {
    unsafe { instance.create_surface(&window) }
}

pub fn generate_default_configuration(
    size: &PhysicalSize<u32>,
    surface: &Surface,
    adapter: &Adapter,
) -> SurfaceConfiguration {
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
            self.renderer.window.size = new_size;

            self.renderer.window.config.width = new_size.width;
            self.renderer.window.config.height = new_size.height;

            self.configure_surface();
        }
    }

    pub fn configure_surface(&mut self) {
        self.renderer
            .window
            .surface
            .configure(&self.renderer.gpu.device, &self.renderer.window.config);
    }
}
