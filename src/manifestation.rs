use log::info;

use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

use crate::{
    manifestation::lift_off::{
        create_surface, create_window, generate_default_configuration, get_adapter, get_device,
        get_headless_device,
    },
    Configuration,
};

pub mod artist;
pub mod lift_off;
pub mod nostalgia;
pub mod painting;

pub trait Renderer {
    fn new(config: &Configuration) -> Self;

    fn device(&mut self) -> &Device;
    fn queue(&mut self) -> &Queue;
}

pub struct Screen {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub size: PhysicalSize<u32>,
    pub surface: Surface,
    pub config: SurfaceConfiguration,

    pub device: Device,
    pub queue: Queue,
}

pub struct GPU {
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

impl Renderer for Screen {
    fn new(config: &Configuration) -> Self {
        let instance = Instance::new(config.backend);
        let (event_loop, window, size) = create_window(config);
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, Some(&surface)));
        let (device, queue) = pollster::block_on(get_device(&adapter));

        let config = generate_default_configuration(&size, &surface, &adapter);
        surface.configure(&device, &config);

        info!("Device name : {}", adapter.get_info().name);

        Self {
            event_loop: Some(event_loop),
            window,
            size,
            surface,
            config,

            device,
            queue,
        }
    }

    fn device(&mut self) -> &Device {
        &self.device
    }

    fn queue(&mut self) -> &Queue {
        &self.queue
    }
}

impl Renderer for GPU {
    fn new(config: &Configuration) -> Self {
        let instance = Instance::new(config.backend);
        let adapter = pollster::block_on(get_adapter(&instance, None));
        let (device, queue) = pollster::block_on(get_headless_device(&adapter));

        info!("Device name : {}", adapter.get_info().name);

        Self {
            adapter,
            device,
            queue,
        }
    }

    fn device(&mut self) -> &Device {
        &self.device
    }

    fn queue(&mut self) -> &Queue {
        &self.queue
    }
}
