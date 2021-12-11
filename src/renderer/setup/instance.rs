use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::Version;

pub struct VglInstance {
    instance: Arc<Instance>,
}

impl VglInstance {
    pub fn new() -> Self {
        let instance = Instance::new(None, Version::V1_1, &vulkano_win::required_extensions(), None).unwrap();

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
