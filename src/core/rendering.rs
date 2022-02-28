use wgpu::{
    Instance,
    Backends,
    SurfaceError,
};


use crate::core::{
    Engine,
    options::IgnitionOptions,
};

pub mod window;
use window::{IgnitionWindow, create_window, create_surface, generate_default_configuration};

pub mod gpu;
use gpu::{IgnitionGPU, get_adapter, get_device};

pub mod command_buffer;
use command_buffer::{create_frame, create_command_encoder, create_render_pass};

pub mod pipeline;
pub mod vertex_buffer;

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
            options: IgnitionOptions { ..Default::default() },

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

            shapes: Vec::new(),
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let (frame, view) = create_frame(self);

        let mut encoder = create_command_encoder(self);
        let mut render_pass = create_render_pass(&mut encoder, &view);

        self.shapes.retain(|shape| { 
            if shape.upgrade().is_none() { return false; }
            true
        });

        for shape in self.shapes.iter() {
            render_pass.set_pipeline(&shape.upgrade().unwrap().borrow().pipeline);
            render_pass.set_vertex_buffer(0, shape.upgrade().unwrap().borrow().vertex_buffer.slice(..));

            render_pass.draw(0..shape.upgrade().unwrap().borrow().vertex_len, 0..1);
        }

        let command_buffer = Some(encoder.finish());
        self.gpu.queue.submit(command_buffer);

        frame.present();

        Ok(())
    }
}
