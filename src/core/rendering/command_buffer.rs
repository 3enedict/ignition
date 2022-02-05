use std::sync::Arc;

use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents, PrimaryAutoCommandBuffer};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::device::{Device, Queue};
use vulkano::render_pass::Framebuffer;


use crate::core::objects::VulkanObject;

pub fn create_command_buffer(
    logical_device: &Arc<Device>,
    queue: &Arc<Queue>,
    pipelines: &Vec<Arc<GraphicsPipeline>>,
    viewport: &Viewport,
    framebuffers: &Vec<Arc<Framebuffer>>,
    swapchain_image: usize,
    objects: &mut Vec<VulkanObject>,
) -> PrimaryAutoCommandBuffer {
    let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];

    let mut builder = AutoCommandBufferBuilder::primary(
        logical_device.clone(),
        queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    builder
        .begin_render_pass(
            framebuffers[swapchain_image].clone(),
            SubpassContents::Inline,
            clear_values,
        )
        .unwrap()
        .set_viewport(0, [viewport.clone()]);

    for object in objects {
        object.draw(&mut builder, pipelines);
    }

    builder
        .end_render_pass()
        .unwrap();

    let command_buffer = builder.build().unwrap();

    command_buffer
}

