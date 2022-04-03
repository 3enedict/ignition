use wgpu::{Backends, Instance, RenderPass};

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use crate::core::{options::Options, Engine};

pub mod window;
use window::{create_surface, create_window, generate_default_configuration, IgnitionWindow};

pub mod gpu;
use gpu::{get_adapter, get_device, IgnitionGPU};

pub mod command_buffer;
use command_buffer::Commands;

pub mod index_buffer;
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

            shapes: Vec::new(),
        }
    }

    pub fn render<'a: 'b, 'b>(&'a self, render_pass: &mut RenderPass<'a>) {
        for shape in self.shapes.iter() {
            if shape.render == true {
                shape.shape.render(render_pass);
            }
        }
    }

    pub fn game_loop<F>(mut self, mut closure: F)
    where
        F: 'static + FnMut(&mut Engine),
    {
        self.window
            .event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| {
                *control_flow = self.options.control_flow;

                match event {
                    Event::WindowEvent {
                        event: WindowEvent::Resized(size),
                        ..
                    } => {
                        self.resize(size);
                    }

                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    Event::RedrawRequested(_) => {
                        let mut commands = match Commands::ignite(&self) {
                            Ok(commands) => commands,
                            Err(wgpu::SurfaceError::Lost) => {
                                self.resize(self.window.size);
                                return;
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit;
                                return;
                            }
                            Err(e) => {
                                eprintln!("{:?}", e);
                                return;
                            }
                        };

                        {
                            let mut render_pass = commands.ignite_render_pass();

                            self.render(&mut render_pass);
                        }

                        commands.execute(&self);
                    }

                    Event::MainEventsCleared => {
                        closure(&mut self);

                        self.window.window.request_redraw();
                    }
                    _ => {}
                }
            });
    }
}
