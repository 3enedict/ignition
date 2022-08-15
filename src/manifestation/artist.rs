use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
};

use crate::Engine;

pub mod commands;
pub mod pipeline;

impl Engine {
    /*
    pub fn game_loop<F>(self, mut closure: F)
    where
        F: 'static + FnMut(&mut Engine),
    {
        self.event_loop(move |engine: &mut Engine| {
            closure(engine);

            engine.render();
        });
    }
    */

    pub fn event_loop<F>(mut self, closure: F)
    where
        F: 'static + FnMut(&mut Engine) -> Result<(), ()>,
    {
        let event_loop = self.renderer.event_loop.take().unwrap();

        match self.config.any_thread {
            true => self.run_return(event_loop, closure),
            false => self.run(event_loop, closure),
        }
    }

    pub fn run<F>(mut self, event_loop: EventLoop<()>, mut closure: F)
    where
        F: 'static + FnMut(&mut Engine) -> Result<(), ()>,
    {
        event_loop.run(move |event, _, control_flow| {
            self.event(event, control_flow, &mut closure);
        });
    }

    pub fn run_return<F>(mut self, mut event_loop: EventLoop<()>, mut closure: F)
    where
        F: 'static + FnMut(&mut Engine) -> Result<(), ()>,
    {
        while self.config.control_flow != ControlFlow::Exit {
            event_loop.run_return(|event, _, control_flow| {
                self.event(event, control_flow, &mut closure);
            });
        }
    }

    pub fn event<F, T>(&mut self, event: Event<T>, control_flow: &mut ControlFlow, closure: &mut F)
    where
        F: 'static + FnMut(&mut Engine) -> Result<(), ()>,
    {
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
            } => {
                *control_flow = ControlFlow::Exit;
                self.config.control_flow = ControlFlow::Exit;
            }

            Event::RedrawRequested(_) => {}

            Event::MainEventsCleared => {
                if closure(self).is_ok() {
                    self.renderer.window.request_redraw();
                }
            }
            _ => {}
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
