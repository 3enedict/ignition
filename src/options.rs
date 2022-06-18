use winit::event_loop::ControlFlow;

pub struct Options {
    pub control_flow: ControlFlow,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            control_flow: ControlFlow::Poll,
        }
    }
}
