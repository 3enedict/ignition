use wgpu::{
    Instance,
    Adapter,
    Device,
    Queue,

    RequestAdapterOptions,

    PowerPreference,
    DeviceDescriptor,
    Features,
    Limits,
};

use crate::core::rendering::window::IgnitionWindow;

pub struct IgnitionGPU {
    pub adapter: Adapter,

    pub device: Device,
    pub queue: Queue,
}

impl IgnitionGPU {
    pub async fn new(instance: &Instance, window: &IgnitionWindow) -> Self {
        let adapter = pollster::block_on(Self::get_adapter(instance, window));

        println!("Device name : {}", adapter.get_info().name);

        let (device, queue) = pollster::block_on(Self::get_device(&adapter));

        Self {
            adapter,

            device,
            queue,
        }
    }

    async fn get_adapter(instance: &Instance, window: &IgnitionWindow) -> Adapter {
        instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&window.surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap()
    }

    async fn get_device(adapter: &Adapter) -> (Device, Queue) {
        adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap()
    }
}
