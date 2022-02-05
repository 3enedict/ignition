use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::Version;

use crate::core::rendering::validation_layers::create_validation_layers;
use crate::core::rendering::validation_layers::setup_debug_callback;

/* IF DEBUG IS ENABLED */

#[cfg(debug_assertions)]
pub fn create_instance() -> Arc<Instance> {
    let mut validation_layers = create_validation_layers();

    let mut required_extensions = vulkano_win::required_extensions();
    required_extensions.ext_debug_utils = true;

    let instance = Instance::new(None, Version::V1_3, &required_extensions, validation_layers).unwrap();

    setup_debug_callback(&instance);

    instance
}






/* IF DEBUG IS DISABLED */

#[cfg(not(debug_assertions))]
pub fn create_instance() -> Arc<Instance> {
    let mut required_extensions = vulkano_win::required_extensions();

    Instance::new(None, Version::V1_3, &required_extensions, None).unwrap()
}

