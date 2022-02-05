use std::sync::Arc;

use vulkano::render_pass::{Framebuffer, RenderPass};
use vulkano::image::{SwapchainImage, ImageAccess};
use vulkano::image::view::ImageView;
use vulkano::swapchain::Swapchain;
use vulkano::pipeline::graphics::viewport::Viewport;

use winit::window::Window;

pub fn create_framebuffers(
    swapchain: &Arc<Swapchain<Window>>,
    swapchain_images: &Vec<Arc<SwapchainImage<Window>>>,
    render_pass: &Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = swapchain_images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    swapchain_images
        .iter()
        .map(|image| {
            let view = ImageView::new(image.clone()).unwrap();
            Framebuffer::start(render_pass.clone())
                .add(view)
                .unwrap()
                .build()
                .unwrap()
        })
    .collect::<Vec<_>>()
}
