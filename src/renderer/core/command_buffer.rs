use std::sync::Arc;


use vulkano::buffer::{CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents, PrimaryAutoCommandBuffer};
use vulkano::pipeline::viewport::Viewport;


use crate::renderer::Vertex;

use crate::renderer::core::VglLogicalDevice;
use crate::renderer::core::VglPipeline;
use crate::renderer::core::VglFramebuffers;
use crate::renderer::core::VglSwapchainImage;

pub struct VglCommandBuffer {
    command_buffer: PrimaryAutoCommandBuffer,
}

impl VglCommandBuffer {
    pub fn new(
        logical_device: &VglLogicalDevice,
        pipeline: &VglPipeline,
        viewport: &Viewport,
        framebuffers: &VglFramebuffers,
        swapchain_image: &VglSwapchainImage,
        vertex_buffer: &Arc<CpuAccessibleBuffer<[Vertex]>>,
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
            .set_viewport(0, [viewport.clone()])
            .bind_pipeline_graphics(pipeline.clone_pipeline())
            .bind_vertex_buffers(0, vertex_buffer.clone())
            .draw(vertex_buffer.len() as u32, 1, 0, 0)
            .unwrap()
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
