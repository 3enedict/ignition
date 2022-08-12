use log::info;

use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, Features, Instance, PowerPreference, PresentMode,
    Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    platform::unix::EventLoopExtUnix,
    window::{Window, WindowBuilder},
};

use crate::{manifestation::Renderer, Configuration};

impl Renderer {
    pub fn new(config: &Configuration) -> Self {
        let (event_loop, window, size) = create_window(config);

        let instance = Instance::new(config.backend);
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, &surface));
        info!("Device name : {}", adapter.get_info().name);
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

pub fn create_window(config: &Configuration) -> (EventLoop<()>, Window, PhysicalSize<u32>) {
    let event_loop = EventLoop::new_any_thread();
    let window = WindowBuilder::new()
        .with_title(config.title)
        .build(&event_loop)
        .expect("Error: Unable to create window - Ignition");

    let size = window.inner_size();

    (event_loop, window, size)
}

pub fn create_surface(instance: &Instance, window: &Window) -> Surface {
    unsafe { instance.create_surface(&window) }
}

pub fn generate_default_configuration(
    size: &PhysicalSize<u32>,
    surface: &Surface,
    adapter: &Adapter,
) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(&adapter)[0],
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
    }
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
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                label: None,
            },
            None,
        )
        .await
        .expect("Error: Failed to create device - Ignition")
}
