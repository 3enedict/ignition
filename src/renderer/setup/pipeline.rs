use std::sync::Arc;

use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Subpass;

use crate::renderer::setup::Vertex;
use crate::renderer::setup::vs;
use crate::renderer::setup::fs;


use crate::renderer::setup::VglLogicalDevice;
use crate::renderer::setup::VglRenderPass;

pub struct VglPipeline {
    pipeline: Arc<GraphicsPipeline>,
}

impl VglPipeline {
    pub fn new(
        logical_device: &VglLogicalDevice,
        render_pass: &VglRenderPass,
        vs: &vs::Shader,
        fs: &fs::Shader,
    ) -> Self {
        let pipeline = Arc::new(
            GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone_render_pass(), 0).unwrap())
            .build(logical_device.clone_logical_device())
            .unwrap(),
        );


        Self {
            pipeline,
        }
    }

    pub fn get_pipeline(&self) -> &Arc<GraphicsPipeline> {
        &self.pipeline
    }

    pub fn clone_pipeline(&self) -> Arc<GraphicsPipeline> {
        self.pipeline.clone()
    }
}
