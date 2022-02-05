use std::sync::Arc;

use vulkano::swapchain as vulkano_swapchain;
use vulkano::swapchain::{Swapchain, AcquireError, SwapchainAcquireFuture};

use winit::window::Window;

pub fn create_swapchain_images(
    swapchain: &Arc<Swapchain<Window>>,
) -> (usize, Option<SwapchainAcquireFuture<Window>>) {
    let (image_num, suboptimal, acquire_future) =
        match vulkano_swapchain::acquire_next_image(swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => return (0, None),
            Err(e) => panic!("Failed to acquire next image: {:?}", e),
        };

    if suboptimal { return (0, None); }

    (image_num, Some(acquire_future))
}
