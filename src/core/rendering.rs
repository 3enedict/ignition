use wgpu::{Backends, Instance};

use crate::core::{options::Options, Engine};

pub mod window;
use window::{create_surface, create_window, generate_default_configuration, IgnitionWindow};

pub mod gpu;
use gpu::{get_adapter, get_device, IgnitionGPU};

pub mod command_buffer;
pub mod index_buffer;
pub mod pipeline;
pub mod vertex_buffer;

impl Engine {
    pub async fn setup_engine(options: Options) -> Engine {
        let (event_loop, window, size) = create_window();

        let instance = Instance::new(Backends::all());
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, &surface));
        println!("Device name : {}", adapter.get_info().name);
        let (device, queue) = pollster::block_on(get_device(&adapter));

        let config = generate_default_configuration(&size, &surface, &adapter);
        surface.configure(&device, &config);

        Self {
            options,

            window: IgnitionWindow {
                event_loop: Some(event_loop),
                window,
                size,

                surface,
                config,
            },

            gpu: IgnitionGPU {
                adapter,

                device,
                queue,
            },
        }
    }
}
