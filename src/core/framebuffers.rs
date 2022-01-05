use std::sync::Arc;

use vulkano::render_pass::Framebuffer;
use vulkano::image::view::ImageView;
use vulkano::image::{SwapchainImage, ImageAccess};
use vulkano::pipeline::graphics::viewport::Viewport;

use winit::window::Window;

use crate::core::VglSwapchain;
use crate::core::VglRenderPass;

pub struct VglFramebuffers {
    framebuffers: Vec<Arc<Framebuffer>>,
}

impl VglFramebuffers {
    pub fn new(
        swapchain: &VglSwapchain,
        render_pass: &VglRenderPass,
        viewport: &mut Viewport,
    ) -> Self {
        let framebuffers = Self::create_framebuffers(
            swapchain.get_images(), 
            render_pass, 
            viewport
        );


        Self {
            framebuffers,
        }
    }

    pub fn recreate_framebuffers(
        &mut self,
        swapchain: &VglSwapchain,
        render_pass: &VglRenderPass,
        viewport: &mut Viewport,
    ) {
        let framebuffers = Self::create_framebuffers(
            swapchain.get_images(), 
            render_pass, 
            viewport
        );

        self.framebuffers = framebuffers;
    }


    fn create_framebuffers(
        images: &[Arc<SwapchainImage<Window>>],
        render_pass: &VglRenderPass,
        viewport: &mut Viewport,
    ) -> Vec<Arc<Framebuffer>> {
        let dimensions = images[0].dimensions().width_height();
        viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

        images
            .iter()
            .map(|image| {
                let view = ImageView::new(image.clone()).unwrap();
                Framebuffer::start(render_pass.clone_render_pass())
                    .add(view)
                    .unwrap()
                    .build()
                    .unwrap()
            })
        .collect::<Vec<_>>()
    }

    pub fn get_framebuffers(&self) -> &Vec<Arc<Framebuffer>> {
        &self.framebuffers
    }

    pub fn clone_framebuffers(&self) -> Vec<Arc<Framebuffer>> {
        self.framebuffers.clone()
    }
}
