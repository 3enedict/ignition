use std::iter::Cloned;
use std::slice::Iter;

use vulkano::instance::debug::DebugCallback;
use vulkano::instance::layers_list;
use vulkano::instance::debug::{MessageType, MessageSeverity};

use crate::renderer::core::VglInstance;

use crate::DEBUG;


pub struct VglValidationLayers<'a> {
    validation_layers: Option<Cloned<Iter<'a, &'a str>>>,

    debug_callback: Option<DebugCallback>,
}

impl VglValidationLayers<'_> {
    pub fn new() -> Self {

        let validation_layers: &[&str] =  &[
            "VK_LAYER_KHRONOS_validation"
        ];

        if DEBUG && !Self::check_validation_layer_support(validation_layers) {
            println!("Validation layers requested, but not available!");

            return Self { validation_layers: None, debug_callback: None };
        }

        Self {
            validation_layers: Some(validation_layers.iter().cloned()),

            debug_callback: None,
        }
    }

    fn check_validation_layer_support(validation_layers: &[&str]) -> bool {
        let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
        validation_layers.iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
    }

    pub fn setup_debug_callback(
        &mut self,
        instance: &VglInstance,
    ) {
        if self.is_enabled() {
            let msg_severity = MessageSeverity {
                error: true,
                warning: true,
                information: false,
                verbose: false,
            };

            let msg_types = MessageType {
                general: true,
                validation: true,
                performance: true,
            };

            self.debug_callback = DebugCallback::new(instance.get_instance(), msg_severity, msg_types, |msg| {
                println!("validation layer: {:?}", msg.description);
            }).ok()
        }
    }

    pub fn is_enabled(
        &self
    ) -> bool {
        self.validation_layers.is_some()
    }

    pub fn get_validation_layers(
        &self,
    ) -> Cloned<Iter<&str>> {
        self.validation_layers.clone().unwrap()
    }
}
