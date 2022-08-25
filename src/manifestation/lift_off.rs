use wgpu::{
    Adapter, Device, Instance, PresentMode, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TextureUsages,
};

use winit::{
    event_loop::EventLoop,
    platform::unix::EventLoopExtUnix,
    window::{Window, WindowBuilder},
};

pub mod headless;
pub mod image;
pub mod screen;

use crate::Configuration;

pub fn create_instance(config: &Configuration) -> Instance {
    Instance::new(config.backend)
}

pub fn create_screen(
    config: &mut Configuration,
    instance: &Instance,
) -> (EventLoop<()>, Window, Surface) {
    let event_loop = create_event_loop(config);
    let window = create_window(&event_loop, config);
    let surface = unsafe { instance.create_surface(&window) };

    config.runtime_config.size = window.inner_size();

    (event_loop, window, surface)
}

pub fn create_event_loop(config: &Configuration) -> EventLoop<()> {
    match config.runtime_config.any_thread {
        false => EventLoop::new(),
        true => EventLoop::new_any_thread(),
    }
}

pub fn create_window(event_loop: &EventLoop<()>, config: &Configuration) -> Window {
    WindowBuilder::new()
        .with_title(config.title)
        .build(event_loop)
        .expect("Error: Unable to create window - Ignition")
}

pub fn get_adapter(
    instance: &Instance,
    config: &Configuration,
    surface: Option<&Surface>,
) -> Adapter {
    let options = RequestAdapterOptions {
        power_preference: config.power_preference,
        compatible_surface: surface,
        force_fallback_adapter: config.force_fallback_adapter,
    };

    pollster::block_on(instance.request_adapter(&options))
        .expect("Error: Failed to find an appropriate adapter - Ignition")
}

pub fn get_device(adapter: &Adapter, config: &Configuration) -> (Device, Queue) {
    pollster::block_on(adapter.request_device(&config.device_options, None))
        .expect("Error: Failed to create device - Ignition")
}

pub fn configure_surface(
    surface: &Surface,
    adapter: &Adapter,
    device: &Device,
    config: &Configuration,
) -> SurfaceConfiguration {
    let config = generate_default_configuration(surface, adapter, &config);
    surface.configure(&device, &config);

    config
}

pub fn generate_default_configuration(
    surface: &Surface,
    adapter: &Adapter,
    config: &Configuration,
) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(&adapter)[0],
        width: config.runtime_config.size.width,
        height: config.runtime_config.size.height,
        present_mode: PresentMode::Fifo,
    }
}
