use std::sync::Arc;

use vulkano::device::Device;
use vulkano::shader::{ShaderModule, ShaderCreationError};

use crate::VglRenderer;
use crate::core::pipeline::VglPipeline;

impl VglRenderer {
    pub fn create_graphics_pipeline(
        &mut self,
        vertex_shader: fn(Arc<Device>) -> Result<Arc<ShaderModule>, ShaderCreationError>,
        fragment_shader: fn(Arc<Device>) -> Result<Arc<ShaderModule>, ShaderCreationError>,
    ) -> usize {
        let vs = vertex_shader(self.logical_device.clone_logical_device()).unwrap();
        let fs = fragment_shader(self.logical_device.clone_logical_device()).unwrap();

        self.pipelines.push(VglPipeline::new(&self.logical_device, &self.render_pass, &vs, &fs));

        self.pipelines.len() - 1
    }
}
