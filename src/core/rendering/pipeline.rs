use std::sync::Arc;

use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::shader::ShaderModule;
use vulkano::device::Device;
use vulkano::render_pass::{RenderPass, Subpass};

use crate::core::objects::vertex::Vertex;

pub fn create_graphics_pipeline(
    logical_device: &Arc<Device>,
    render_pass: &Arc<RenderPass>,
    vs: &Arc<ShaderModule>,
    fs: &Arc<ShaderModule>,
) -> Arc<GraphicsPipeline> {
    GraphicsPipeline::start()
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(logical_device.clone())
        .unwrap()
}
