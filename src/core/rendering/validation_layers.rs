use std::sync::Arc;
use std::iter::Cloned;
use std::slice::Iter;

use vulkano::instance::debug::DebugCallback;
use vulkano::instance::{Instance, layers_list};
use vulkano::instance::debug::{MessageType, MessageSeverity};

pub fn create_validation_layers<'a>() -> Cloned<Iter<'a, &'a str>> {
    let validation_layers: &[&str] =  &[
        "VK_LAYER_KHRONOS_validation"
    ];

    if !check_validation_layer_support(validation_layers) { println!("Warning : Validation layers are not available!"); }

    validation_layers.iter().cloned()
}

fn check_validation_layer_support(validation_layers: &[&str]) -> bool {
    let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
    validation_layers.iter()
        .all(|layer_name| layers.contains(&layer_name.to_string()))
}

pub fn setup_debug_callback(instance: &Arc<Instance>) {
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

    DebugCallback::new(instance, msg_severity, msg_types, |msg| {
        println!("validation layer: {:?}", msg.description);

        if msg.severity.error == true || msg.severity.warning == true {
            panic!();
        }
    }).ok();
}
