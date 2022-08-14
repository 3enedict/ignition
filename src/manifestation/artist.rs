use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use crate::Engine;

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
                *control_flow = self.config.control_flow;

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

                    Event::RedrawRequested(_) => {}

                    Event::MainEventsCleared => {
                        closure(&mut self);

                        self.renderer.window.request_redraw();
                    }
                    _ => {}
                }
            });
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
