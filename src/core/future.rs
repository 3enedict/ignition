use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};


use crate::core::VglLogicalDevice;
use crate::core::VglSwapchain;
use crate::core::VglSwapchainImage;
use crate::core::VglCommandBuffer;

pub struct VglFuture {
    future: Option<Box<dyn GpuFuture>>,
}

impl VglFuture {
    pub fn new(
        logical_device: &VglLogicalDevice,
    ) -> Self {
        let future = Self::create_brand_new_future(logical_device);

        Self {
            future,
        }
    }

    pub fn update_future(
        &mut self,
        logical_device: &VglLogicalDevice,
        swapchain: &VglSwapchain,
        mut swapchain_image: VglSwapchainImage,
        command_buffer: VglCommandBuffer,
    ) -> bool {
        let future = self.future.take().unwrap()
            .join(swapchain_image.get_acquire_future())
            .then_execute(logical_device.clone_queue(), command_buffer.get_command_buffer())
            .unwrap()
            .then_swapchain_present(logical_device.clone_queue(), swapchain.clone_swapchain(), swapchain_image.get_image_num())
            .then_signal_fence_and_flush();

        let mut recreate_swapchain = false;

        match future {
            Ok(future) => {
                self.future = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                recreate_swapchain = true;
                self.future = Self::create_brand_new_future(logical_device);
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.future = Self::create_brand_new_future(logical_device);
            }
        }

        recreate_swapchain
    }

    fn create_brand_new_future(
        logical_device: &VglLogicalDevice,
    ) -> Option<Box<dyn GpuFuture>> {
        Some(sync::now(logical_device.clone_logical_device()).boxed())
    }

    pub fn cleanup(&mut self) {
        self.future.as_mut().unwrap().cleanup_finished();
    }

    pub fn get_future(&self) -> &Option<Box<dyn GpuFuture>> {
        &self.future
    }
}
