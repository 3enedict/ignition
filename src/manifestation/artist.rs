pub mod command_buffer;
pub mod pipeline;
use wgpu::RenderPass;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use crate::{
    manifestation::{artist::command_buffer::Commands, silhouette::Renderable},
    Engine,
};

impl Engine {
    pub fn game_loop<F>(mut self, mut closure: F)
    where
        F: 'static + FnMut(&mut Engine),
    {
        self.renderer
            .event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| {
                *control_flow = self.parameters.control_flow;

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
                                self.resize(self.renderer.size);
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

                        self.renderer.window.request_redraw();
                    }
                    _ => {}
                }
            });
    }

    pub fn render<'a>(&'a mut self, render_pass: &mut RenderPass<'a>) {
        if self.scene.component_pool_exists::<Box<dyn Renderable>>() {
            let shapes = self.scene.get::<Box<dyn Renderable>>();

            for shape in shapes.iter() {
                shape.render(render_pass);
            }
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.renderer.size = new_size;

            self.renderer.config.width = new_size.width;
            self.renderer.config.height = new_size.height;

            self.configure_surface();
        }
    }

    pub fn configure_surface(&mut self) {
        self.renderer
            .surface
            .configure(&self.renderer.device, &self.renderer.config);
    }
}
