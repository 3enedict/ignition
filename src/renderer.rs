use wgpu::{Backends, Instance};

use crate::renderer::core::gpu::{get_adapter, get_device, GPU};
use crate::renderer::core::window::{
    create_surface, create_window, generate_default_configuration, Window,
};

pub mod core;
pub mod ecs;
pub mod shapes;

pub struct Renderer {
    window: Window,
    gpu: GPU,
}

impl Renderer {
    pub fn new() -> Self {
        let (event_loop, window, size) = create_window();

        let instance = Instance::new(Backends::all());
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, &surface));
        println!("Device name : {}", adapter.get_info().name);
        let (device, queue) = pollster::block_on(get_device(&adapter));

        let config = generate_default_configuration(&size, &surface, &adapter);
        surface.configure(&device, &config);

        Self {
            window: Window {
                event_loop: Some(event_loop),
                window,
                size,

                surface,
                config,
            },

            gpu: GPU {
                adapter,

                device,
                queue,
            },
        }
    }
}
