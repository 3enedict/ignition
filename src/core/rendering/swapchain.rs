use std::sync::Arc;

use vulkano::swapchain::{Swapchain, SwapchainCreationError, Surface};
use vulkano::image::{SwapchainImage, ImageUsage};
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, Queue};

use winit::window::Window;

pub fn create_swapchain(
    surface: &Arc<Surface<Window>>,
    physical_device: &PhysicalDevice,
    logical_device: &Arc<Device>,
    queue: &Arc<Queue>,
) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    let (swapchain, images) = {
        let caps = surface.capabilities(physical_device.clone()).unwrap();
        let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        let dimensions: [u32; 2] = surface.window().inner_size().into();
        Swapchain::start(logical_device.clone(), surface.clone())
            .num_images(caps.min_image_count)
            .format(format)
            .dimensions(dimensions)
            .usage(ImageUsage::color_attachment())
            .sharing_mode(queue)
            .composite_alpha(composite_alpha)
            .build()
            .unwrap()
    };


    (swapchain, images)
}

pub fn recreate_swapchain(
    surface: &Arc<Surface<Window>>,
    swapchain: &Arc<Swapchain<Window>>,
) -> (bool, Option<(Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>)>) {
    let recreate_swapchain = false;

    let dimensions: [u32; 2] = surface.window().inner_size().into();
    let (new_swapchain, new_images) =
        match swapchain.recreate().dimensions(dimensions).build() {
            Ok(r) => r,
            Err(SwapchainCreationError::UnsupportedDimensions) => return (true, None),
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };

    (false, Some((new_swapchain, new_images)))
}
