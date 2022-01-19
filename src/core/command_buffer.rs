use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents, PrimaryAutoCommandBuffer};
use vulkano::pipeline::graphics::viewport::Viewport;


use crate::core::VglLogicalDevice;
use crate::core::VglPipeline;
use crate::core::VglFramebuffers;
use crate::core::VglSwapchainImage;

use crate::objects::VulkanObject;

pub struct VglCommandBuffer {
    command_buffer: PrimaryAutoCommandBuffer,
}

impl VglCommandBuffer {
    pub fn new(
        logical_device: &VglLogicalDevice,
        pipelines: &Vec<VglPipeline>,
        viewport: &Viewport,
        framebuffers: &VglFramebuffers,
        swapchain_image: &VglSwapchainImage,
        objects: &mut Vec<VulkanObject>,
    ) -> Self {
        let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];

        let mut builder = AutoCommandBufferBuilder::primary(
            logical_device.clone_logical_device(),
            logical_device.get_queue().family(),
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap();

        builder
            .begin_render_pass(
                framebuffers.get_framebuffers()[swapchain_image.get_image_num()].clone(),
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

        Self {
            command_buffer,
        }
    }

    pub fn get_command_buffer(self) -> PrimaryAutoCommandBuffer {
        self.command_buffer
    }
}
