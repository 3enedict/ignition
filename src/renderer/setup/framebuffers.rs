use std::sync::Arc;

use vulkano::render_pass::{Framebuffer, FramebufferAbstract};
use vulkano::image::view::ImageView;
use vulkano::image::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;

use winit::window::Window;

use crate::renderer::setup::VglSwapchain;
use crate::renderer::setup::VglRenderPass;

pub struct VglFramebuffers {
    framebuffers: Vec<Arc<dyn FramebufferAbstract>>,
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
    ) -> Vec<Arc<dyn FramebufferAbstract>> {
        let dimensions = images[0].dimensions();
        viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

        images
            .iter()
            .map(|image| {
                let view = ImageView::new(image.clone()).unwrap();
                Arc::new(
                    Framebuffer::start(render_pass.clone_render_pass())
                    .add(view)
                    .unwrap()
                    .build()
                    .unwrap(),
                ) as Arc<dyn FramebufferAbstract>
            })
        .collect::<Vec<_>>()
    }

    pub fn get_framebuffers(&self) -> &Vec<Arc<dyn FramebufferAbstract>> {
        &self.framebuffers
    }

    pub fn clone_framebuffers(&self) -> Vec<Arc<dyn FramebufferAbstract>> {
        self.framebuffers.clone()
    }
}
