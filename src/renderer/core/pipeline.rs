use std::sync::Arc;

use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::shader::ShaderModule;
use vulkano::render_pass::Subpass;

use crate::object::vertex::Vertex;

use crate::renderer::core::VglLogicalDevice;
use crate::renderer::core::VglRenderPass;

pub struct VglPipeline {
    pipeline: Arc<GraphicsPipeline>,
}

impl VglPipeline {
    pub fn new(
        logical_device: &VglLogicalDevice,
        render_pass: &VglRenderPass,
        vs: &Arc<ShaderModule>,
        fs: &Arc<ShaderModule>,
    ) -> Self {
        let pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .input_assembly_state(InputAssemblyState::new())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .render_pass(Subpass::from(render_pass.clone_render_pass(), 0).unwrap())
            .build(logical_device.clone_logical_device())
            .unwrap();


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
