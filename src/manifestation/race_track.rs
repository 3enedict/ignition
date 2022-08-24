use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
};

use crate::{manifestation::lift_off::screen::Screen, Engine};

impl Engine<Screen> {
    pub fn event_loop(self, closure: impl FnMut(&mut Engine<Screen>) -> Result<(), ()> + 'static) {
        match self.config.any_thread {
            true => self.run_return(closure),
            false => self.run(closure),
        }
    }

    pub fn run(mut self, mut closure: impl FnMut(&mut Engine<Screen>) -> Result<(), ()> + 'static) {
        self.take_event_loop().run(move |event, _, control_flow| {
            self.event(event, control_flow, &mut closure);
        });
    }

    pub fn run_return(mut self, mut closure: impl FnMut(&mut Engine<Screen>) -> Result<(), ()>) {
        self.take_event_loop().run_return(|event, _, control_flow| {
            self.event(event, control_flow, &mut closure);
        });
    }

    pub fn run_once(&mut self, mut closure: impl FnMut(&mut Engine<Screen>) -> Result<(), ()>) {
        let mut event_loop = self.take_event_loop();

        event_loop.run_return(|event, _, control_flow| {
            self.event(event, control_flow, &mut |engine: &mut Engine<Screen>| {
                engine.config.control_flow = ControlFlow::Exit;
                closure(engine)
            });
        });

        self.renderer.event_loop = Some(event_loop);
    }

    pub fn event<T>(
        &mut self,
        event: Event<T>,
        control_flow: &mut ControlFlow,
        closure: &mut impl FnMut(&mut Engine<Screen>) -> Result<(), ()>,
    ) {
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

    pub fn take_event_loop(&mut self) -> EventLoop<()> {
        self.renderer.event_loop.take().unwrap()
    }
}
