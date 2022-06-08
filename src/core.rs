use wgpu::{Backends, Instance};

use crate::core::options::Options;
use crate::core::rendering::gpu::{get_adapter, get_device, IgnitionGPU};
use crate::core::rendering::window::{
    create_surface, create_window, generate_default_configuration, IgnitionWindow,
};
use crate::ecs::IgnitionScene;
use crate::Engine;

pub mod ecs;
pub mod options;
pub mod rendering;
pub mod shapes;

impl Engine {
    pub async fn setup_engine() -> Engine {
        let (event_loop, window, size) = create_window();

        let instance = Instance::new(Backends::all());
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, &surface));
        println!("Device name : {}", adapter.get_info().name);
        let (device, queue) = pollster::block_on(get_device(&adapter));

        let config = generate_default_configuration(&size, &surface, &adapter);
        surface.configure(&device, &config);

        Self {
            options: Options::default(),

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

            scene: IgnitionScene::new(),
        }
    }
}
