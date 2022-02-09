use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
};

pub struct VglWindow {
    pub event_loop: Option<EventLoop<()>>, 
    pub window: Window, 
    pub size: PhysicalSize<u32>,
}

pub fn create_window() -> VglWindow {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();


    let size = window.inner_size();

    VglWindow { event_loop: Some(event_loop), window, size }
}
