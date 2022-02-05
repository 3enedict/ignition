use std::sync::Arc;

use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Swapchain;
use vulkano::device::Device;

use winit::window::Window;

pub fn create_render_pass(
    logical_device: &Arc<Device>,
    swapchain: &Arc<Swapchain<Window>>,
) -> Arc<RenderPass> {
    vulkano::single_pass_renderpass!(
        logical_device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    )
        .unwrap()
}
