use winit::event_loop::ControlFlow;

#[derive(Builder, Debug, PartialEq)]
pub struct Options {
    #[builder(default = "ControlFlow::Poll")]
    pub control_flow: ControlFlow,

    #[builder(default = "false")]
    pub any_thread: bool,
}
