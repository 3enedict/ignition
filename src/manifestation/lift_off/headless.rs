use log::info;
use wgpu::{Device, Queue};

use crate::{
    liberty::Configuration,
    manifestation::{
        lift_off::{create_instance, get_adapter, get_headless_device},
        Renderer,
    },
};

pub struct Headless {
    pub device: Device,
    pub queue: Queue,
}

impl Renderer for Headless {
    fn new(config: &mut Configuration) -> Self {
        let instance = create_instance(config);
        let adapter = get_adapter(&instance, None);
        let (device, queue) = get_headless_device(&adapter);

        info!("Device name : {}", adapter.get_info().name);

        Self { device, queue }
    }

    fn device(&self) -> &Device {
        &self.device
    }

    fn queue(&self) -> &Queue {
        &self.queue
    }

    fn device_mut(&mut self) -> &mut Device {
        &mut self.device
    }

    fn queue_mut(&mut self) -> &mut Queue {
        &mut self.queue
    }
}
