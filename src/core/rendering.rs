use wgpu::{
    Instance,
    Backends,

    SurfaceError,
    TextureViewDescriptor,
};


use crate::core::Engine;

pub mod window;
use window::{IgnitionWindow, create_window, create_surface, generate_default_configuration};

pub mod gpu;
use gpu::{IgnitionGPU, get_adapter, get_device};

pub mod commands;
use commands::create_command_buffer;

pub mod pipeline;

impl Engine {
    pub async fn setup_engine() -> Self {
        env_logger::init();


        let (event_loop, window, size) = create_window();

        let instance = Instance::new(Backends::all());
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, &surface));
        println!("Device name : {}", adapter.get_info().name);
        let (device, queue) = pollster::block_on(get_device(&adapter));

        let config = generate_default_configuration(&size, &surface, &adapter);
        surface.configure(&device, &config);

        Self {
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

            pipelines: Vec::new(),
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let frame = self.window.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let command_buffer = create_command_buffer(self, &view);

        self.gpu.queue.submit(command_buffer);
        frame.present();

        Ok(())
    }
}
