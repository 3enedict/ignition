use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::Version;

use crate::core::rendering::validation_layers::VglValidationLayers;

pub struct VglInstance {
    instance: Arc<Instance>,
}

impl VglInstance {
    pub fn new(
        validation_layers: &VglValidationLayers,
    ) -> Self {
        let instance;
        let mut required_extensions = vulkano_win::required_extensions();

        if validation_layers.is_enabled() {
            required_extensions.ext_debug_utils = true;
            instance = Instance::new(None, Version::V1_1, &required_extensions, validation_layers.get_validation_layers()).unwrap();
        } else {
            instance = Instance::new(None, Version::V1_1, &required_extensions, None).unwrap();
        }


        Self {
            instance,
        }
    }

    pub fn get_instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub fn clone_instance(&self) -> Arc<Instance> {
        self.instance.clone()
    }
}
