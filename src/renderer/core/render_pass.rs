use std::sync::Arc;

use vulkano::render_pass::RenderPass;

use crate::renderer::core::VglLogicalDevice;
use crate::renderer::core::VglSwapchain;

pub struct VglRenderPass {
    render_pass: Arc<RenderPass>,
}

impl VglRenderPass {
    pub fn new(
        logical_device: &VglLogicalDevice,
        swapchain: &VglSwapchain,
    ) -> Self {
        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(
                logical_device.clone_logical_device(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: swapchain.get_swapchain().format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
            }
        )
            .unwrap(),
        );

        Self {
            render_pass,
        }
    }

    pub fn get_render_pass(&self) -> &Arc<RenderPass> {
        &self.render_pass
    }

    pub fn clone_render_pass(&self) -> Arc<RenderPass> {
        self.render_pass.clone()
    }
}
