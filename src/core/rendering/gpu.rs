use wgpu::{
    Instance,
    Adapter,
    Device,
    Queue,
    Surface,

    RequestAdapterOptions,

    PowerPreference,
    DeviceDescriptor,
    Features,
    Limits,
};

pub struct IgnitionGPU {
    pub adapter: Adapter,

    pub device: Device,
    pub queue: Queue,
}

pub async fn get_adapter(instance: &Instance, surface: &Surface) -> Adapter {
    instance.request_adapter(
        &RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await.unwrap()
}

pub async fn get_device(adapter: &Adapter) -> (Device, Queue) {
    adapter.request_device(
        &DeviceDescriptor {
            features: Features::empty(),
            limits: Limits::default(),
            label: None,
        },
        None,
    ).await.unwrap()
}
