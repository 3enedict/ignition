use std::sync::Arc;

use vulkano::swapchain::{Swapchain, SwapchainCreationError};
use vulkano::image::{SwapchainImage, ImageUsage};

use winit::window::Window;

use crate::renderer::core::VglSurface;
use crate::renderer::core::VglPhysicalDevice;
use crate::renderer::core::VglLogicalDevice;

pub struct VglSwapchain {
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
}

impl VglSwapchain {
    pub fn new(
        surface: &VglSurface,
        physical_device: &VglPhysicalDevice,
        logical_device: &VglLogicalDevice,
    ) -> Self {
        let (swapchain, images) = {
            let caps = surface.get_surface().capabilities(physical_device.get_physical_device()).unwrap();
            let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            let dimensions: [u32; 2] = surface.get_surface().window().inner_size().into();
            Swapchain::start(logical_device.clone_logical_device(), surface.clone_surface())
                .num_images(caps.min_image_count)
                .format(format)
                .dimensions(dimensions)
                .usage(ImageUsage::color_attachment())
                .sharing_mode(logical_device.get_queue())
                .composite_alpha(composite_alpha)
                .build()
                .unwrap()
        };


        Self {
            swapchain,
            images,
        }
    }

    pub fn recreate_swapchain(
        &mut self,
        surface: &VglSurface,
    ) -> bool {
        let dimensions: [u32; 2] = surface.get_surface().window().inner_size().into();
        let (new_swapchain, new_images) =
            match self.swapchain.recreate().dimensions(dimensions).build() {
                Ok(r) => r,
                Err(SwapchainCreationError::UnsupportedDimensions) => return true,
                Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
            };

        self.swapchain = new_swapchain;
        self.images = new_images;

        return false;
    }

    pub fn get_swapchain(&self) -> &Arc<Swapchain<Window>> {
        &self.swapchain
    }

    pub fn clone_swapchain(&self) -> Arc<Swapchain<Window>> {
        self.swapchain.clone()
    }

    pub fn get_images(&self) -> &Vec<Arc<SwapchainImage<Window>>> {
        &self.images
    }

    pub fn clone_images(&self) -> Vec<Arc<SwapchainImage<Window>>> {
        self.images.clone()
    }
}
