use wgpu::RenderPass;

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use crate::Engine;

pub mod command_buffer;
pub mod gpu;
pub mod window;
use command_buffer::Commands;

pub mod index_buffer;
pub mod pipeline;
pub mod vertex_buffer;

impl Engine {
    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        for i in 0..self.scene.shape_component.len() {
            if self.scene.render_component[i].unwrap() == true {
                self.scene.shape_component[i]
                    .as_ref()
                    .unwrap()
                    .render(render_pass);
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
