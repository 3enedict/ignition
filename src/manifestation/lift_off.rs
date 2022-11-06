use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, Instance, PowerPreference, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};

use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn create_instance() -> Instance {
    Instance::new(Backends::all())
}

pub fn create_screen(instance: &Instance) -> (EventLoop<()>, Window, Surface) {
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);
    let surface = unsafe { instance.create_surface(&window) };

    (event_loop, window, surface)
}

pub fn create_window(event_loop: &EventLoop<()>) -> Window {
    WindowBuilder::new()
        .build(event_loop)
        .expect("Error: Unable to create window - Ignition")
}

pub fn get_adapter(instance: &Instance, surface: &Surface) -> Adapter {
    let options = RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        compatible_surface: Some(surface),
        force_fallback_adapter: false,
    };

    pollster::block_on(instance.request_adapter(&options))
        .expect("Error: Failed to find an appropriate adapter - Ignition")
}

pub fn get_device(adapter: &Adapter) -> (Device, Queue) {
    pollster::block_on(adapter.request_device(&DeviceDescriptor::default(), None))
        .expect("Error: Failed to create device - Ignition")
}

pub fn configure_surface(
    surface: &Surface,
    adapter: &Adapter,
    device: &Device,
) -> SurfaceConfiguration {
    let config = generate_default_configuration(surface, adapter);
    surface.configure(&device, &config);

    config
}

pub fn generate_default_configuration(
    surface: &Surface,
    adapter: &Adapter,
) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(&adapter)[0],
        width: 1920,
        height: 1080,
        present_mode: PresentMode::Fifo,
    }
}
