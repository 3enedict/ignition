use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, Queue,
    RequestAdapterOptions, Surface,
};

pub struct IgnitionGPU {
    pub adapter: Adapter,

    pub device: Device,
    pub queue: Queue,
}

pub async fn get_adapter(instance: &Instance, surface: &Surface) -> Adapter {
    instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Error: Failed to find an appropriate adapter - Ignition")
}

pub async fn get_device(adapter: &Adapter) -> (Device, Queue) {
    adapter
        .request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .expect("Error: Failed to create device - Ignition")
}
