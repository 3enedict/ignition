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
        let adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&window.surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let adapter_info = adapter.get_info();
        println!("Device name : {}", adapter_info.name);

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();
        
        Self {
            adapter,

            device,
            queue,
        }
    }
}
