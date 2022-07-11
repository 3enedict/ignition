use wgpu::{Adapter, Backends, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

pub mod apex;
pub mod artist;
pub mod life;
pub mod lift_off;
pub mod nostalgia;
pub mod silhouette;

use crate::{
    liberty::Parameters,
    manifestation::lift_off::{
        create_surface, create_window, generate_default_configuration, get_adapter, get_device,
    },
};

pub struct Renderer {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub size: PhysicalSize<u32>,

    pub surface: Surface,
    pub config: SurfaceConfiguration,

    pub adapter: Adapter,

    pub device: Device,
    pub queue: Queue,
}

impl Renderer {
    pub fn new(parameters: &Parameters) -> Self {
        let (event_loop, window, size) = create_window(parameters);

        let instance = Instance::new(Backends::all());
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, &surface));
        println!("Device name : {}", adapter.get_info().name);
        let (device, queue) = pollster::block_on(get_device(&adapter));

        let config = generate_default_configuration(&size, &surface, &adapter);
        surface.configure(&device, &config);

        Self {
            event_loop: Some(event_loop),
            window,
            size,
            surface,
            config,

            adapter,
            device,
            queue,
        }
    }
}
