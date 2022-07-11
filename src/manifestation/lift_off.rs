use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, PresentMode,
    Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    platform::unix::EventLoopExtUnix,
    window::{Window, WindowBuilder},
};

use crate::liberty::Parameters;

pub fn create_window(parameters: &Parameters) -> (EventLoop<()>, Window, PhysicalSize<u32>) {
    let event_loop = EventLoop::new_any_thread();
    let window = WindowBuilder::new()
        .with_title(parameters.window_title.clone())
        .build(&event_loop)
        .unwrap();

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
        format: surface.get_preferred_format(adapter).unwrap(),
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
                limits: Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .expect("Error: Failed to create device - Ignition")
}
