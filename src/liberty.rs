use wgpu::{Backends, Instance};

use winit::{event_loop::EventLoop, platform::unix::EventLoopExtUnix, window::WindowBuilder};

use crate::{
    manifestation::{
        lift_off::{create_surface, generate_default_configuration, get_adapter, get_device},
        Renderer,
    },
    Engine,
};

pub struct EngineBuilder {
    pub window: WindowBuilder,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self {
            window: WindowBuilder::new(),
        }
    }

    pub fn ignite(self) -> Engine {
        let event_loop = EventLoop::new_any_thread();
        let window = self.window.build(&event_loop).unwrap();

        let size = window.inner_size();

        let instance = Instance::new(Backends::all());
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, &surface));
        println!("Device name : {}", adapter.get_info().name);
        let (device, queue) = pollster::block_on(get_device(&adapter));

        let config = generate_default_configuration(&size, &surface, &adapter);
        surface.configure(&device, &config);

        Engine {
            renderer: Renderer {
                event_loop: Some(event_loop),
                window,
                size,
                surface,
                config,

                adapter,
                device,
                queue,
            },
        }
    }

    pub fn title<G: Into<String>>(mut self, title: G) -> Self {
        self.window = self.window.with_title(title.into());
        self
    }
}
