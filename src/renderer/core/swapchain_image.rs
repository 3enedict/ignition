use vulkano::swapchain as vulkano_swapchain;
use vulkano::swapchain::{AcquireError, SwapchainAcquireFuture};

use winit::window::Window;

use crate::renderer::core::VglSwapchain;

pub struct VglSwapchainImage {
    image_num: usize,
    acquire_future: Option<SwapchainAcquireFuture<Window>>,
}

impl VglSwapchainImage {
    pub fn new(
        swapchain: &VglSwapchain,
    ) -> Self {
        let (image_num, suboptimal, acquire_future) =
            match vulkano_swapchain::acquire_next_image(swapchain.clone_swapchain(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => return Self::generate_empty_swapchain_image_object(),
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

        if suboptimal { return Self::generate_empty_swapchain_image_object(); }

        Self {
            image_num, 
            acquire_future: Some(acquire_future),
        }
    }

    fn generate_empty_swapchain_image_object() -> Self {
        Self { image_num: 0, acquire_future: None }
    }

    pub fn get_image_num(&self) -> usize {
        self.image_num
    }

    // This function can only be called once because self.acquire_future is given vulkano.
    pub fn get_acquire_future(&mut self) -> SwapchainAcquireFuture<Window> {
        let acquire_future = self.acquire_future.take().unwrap();
        self.acquire_future = None;

        acquire_future
    }

    pub fn suboptimal(&self) -> bool {
        self.acquire_future.is_none()
    }
}
