use winit::event_loop::ControlFlow;

use crate::{logger, Engine};

#[derive(Builder, Debug, PartialEq)]
#[builder(default)]
pub struct Parameters {
    pub control_flow: ControlFlow,

    pub window_title: String,
}

impl Default for Parameters {
    fn default() -> Parameters {
        Parameters {
            control_flow: ControlFlow::Poll,

            window_title: String::from("The Dark Web"),
        }
    }
}

impl ParametersBuilder {
    pub fn ignite(&mut self) -> Engine {
        logger();
        Engine::setup_engine(self.build().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::{liberty::ParametersBuilder, Engine};
    use winit::event_loop::ControlFlow;

    #[test]
    fn instantiating_parameters_with_engine_returns_correct_defaults() {
        testing_logger::setup(); // This is only to prevent env_logger to start up

        let engine = Engine::ignite();
        let default_parameters = ParametersBuilder::default().build().unwrap();

        assert_eq!(engine.parameters, default_parameters);
    }

    #[test]
    fn changing_parameters_in_engine_returns_correct_parameters() {
        testing_logger::setup(); // This is only to prevent env_logger to start up

        let engine = Engine::parameters()
            .control_flow(ControlFlow::Wait)
            .ignite();

        let default_parameters = ParametersBuilder::default()
            .control_flow(ControlFlow::Wait)
            .build()
            .unwrap();

        assert_eq!(engine.parameters, default_parameters);
    }
}
