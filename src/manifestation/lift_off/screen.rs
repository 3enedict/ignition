use log::info;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::{event_loop::EventLoop, window::Window};

use crate::{
    liberty::Configuration,
    manifestation::{
        lift_off::{
            configure_surface, create_instance, create_screen, get_adapter, get_device,
            headless::Headless,
        },
        Renderer,
    },
};

pub struct Screen {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,

    pub surface: Surface,
    pub config: SurfaceConfiguration,

    pub gpu: Headless,
}

impl Renderer for Screen {
    fn new(config: &mut Configuration) -> Self {
        let instance = create_instance(config);
        let (event_loop, window, surface) = create_screen(config, &instance);

        let adapter = get_adapter(&instance, Some(&surface));
        let (device, queue) = get_device(&adapter);

        let config = configure_surface(&surface, &adapter, &device, &config);

        info!("Device name : {}", adapter.get_info().name);

        Self {
            event_loop: Some(event_loop),
            window,

            surface,
            config,

            gpu: Headless { device, queue },
        }
    }

    fn device(&self) -> &Device {
        &self.gpu.device
    }

    fn queue(&self) -> &Queue {
        &self.gpu.queue
    }

    fn device_mut(&mut self) -> &mut Device {
        &mut self.gpu.device
    }

    fn queue_mut(&mut self) -> &mut Queue {
        &mut self.gpu.queue
    }
}
