use wgpu::{
    Instance,
    Surface,

    SurfaceConfiguration,
    TextureUsages,
    TextureFormat,
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

impl IgnitionWindow {
    pub fn new(instance: &Instance) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(&window) };

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
        };

        Self { 
            event_loop: Some(event_loop), 

            window, 
            size,

            surface,
            config,
        }
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
