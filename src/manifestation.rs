use log::info;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::{event_loop::EventLoop, window::Window};

pub mod artist;
pub mod lift_off;
pub mod nostalgia;
pub mod race_track;

use crate::manifestation::lift_off::{
    configure_surface, create_instance, create_screen, get_adapter, get_device,
};

pub struct Screen {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,

    pub surface: Surface,
    pub config: SurfaceConfiguration,

    pub device: Device,
    pub queue: Queue,
}

impl Screen {
    pub fn new() -> Self {
        let instance = create_instance();
        let (event_loop, window, surface) = create_screen(&instance);

        let adapter = get_adapter(&instance, &surface);
        let (device, queue) = get_device(&adapter);

        let config = configure_surface(&surface, &adapter, &device);

        info!("Device name : {}", adapter.get_info().name);

        Self {
            event_loop: Some(event_loop),
            window,

            surface,
            config,

            device,
            queue,
        }
    }
}
