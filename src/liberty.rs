use winit::{dpi::PhysicalSize, event_loop::ControlFlow};

pub struct RuntimeConfiguration {
    pub control_flow: ControlFlow,
    pub any_thread: bool,

    pub size: PhysicalSize<u32>,
}

impl Default for RuntimeConfiguration {
    fn default() -> Self {
        Self {
            control_flow: ControlFlow::Poll,
            any_thread: false,

            size: PhysicalSize {
                width: 1920,
                height: 1080,
            },
        }
    }
}
