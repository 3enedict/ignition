use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct VglRenderer {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
}

impl VglRenderer {
    pub fn new() -> Self {
        env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        Self {
            event_loop: Some(event_loop),
            window,
        }
    }
}
