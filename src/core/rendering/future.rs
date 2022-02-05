use std::sync::Arc;

use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use vulkano::device::Device;
use vulkano::swapchain::SwapchainAcquireFuture;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;

use winit::window::Window;

use crate::core::VglRenderer;

pub fn create_future(
    logical_device: &Arc<Device>,
) -> Option<Box<dyn GpuFuture>> {
    Some(sync::now(logical_device.clone()).boxed())
}

pub fn update_future(
    renderer: &mut VglRenderer,
    acquire_future: Option<SwapchainAcquireFuture<Window>>,
    swapchain_image: usize,
    command_buffer: PrimaryAutoCommandBuffer,
) -> bool {
    let future = renderer.future
        .take()
        .unwrap()
        .join(acquire_future.unwrap())
        .then_execute(renderer.queue.clone(), command_buffer)
        .unwrap()
        .then_swapchain_present(renderer.queue.clone(), renderer.swapchain.clone(), swapchain_image)
        .then_signal_fence_and_flush();

    let mut recreate_swapchain = false;
    let updated_future;

    match future {
        Ok(future) => {
            updated_future = Some(future.boxed());
        }
        Err(FlushError::OutOfDate) => {
            recreate_swapchain = true;
            updated_future = create_future(&renderer.logical_device);
        }
        Err(e) => {
            println!("Failed to flush future: {:?}", e);
            updated_future = create_future(&renderer.logical_device);
        }
    }

    renderer.future = updated_future;

    recreate_swapchain
}


pub fn cleanup_future(future: &mut Option<Box<dyn GpuFuture>>) {
    future.as_mut().unwrap().cleanup_finished();
}
