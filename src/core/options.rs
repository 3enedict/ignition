use winit::event_loop::ControlFlow;

pub struct IgnitionOptions {
    pub control_flow: ControlFlow,
}

impl Default for IgnitionOptions {
    fn default() -> IgnitionOptions {
        IgnitionOptions {
            control_flow: ControlFlow::Poll,
        }
    }
}
