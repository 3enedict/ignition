use std::sync::Arc;

use vulkano::device::{Device, DeviceExtensions, Queue, Features};


use crate::renderer::core::VglPhysicalDevice;

// Note: The physical device is stored as an index to avoid lifetime problems within classes
//       The same is true for the instance.
pub struct VglLogicalDevice {
  logical_device: Arc<Device>,
  queue: Arc<Queue>,
}

impl VglLogicalDevice {
    pub fn new(
        device_extensions: &DeviceExtensions,
        physical_device: &VglPhysicalDevice,
    ) -> Self {
        let (logical_device, mut queues) = Device::new(
            physical_device.get_physical_device(),
            &Features::none(),
            &physical_device.get_physical_device()
            .required_extensions()
            .union(&device_extensions),
            [(physical_device.get_queue_family(), 0.5)].iter().cloned(),
        )
            .unwrap();

        let queue = queues.next().unwrap();

        Self {
            logical_device,
            queue,
        }
    }

    pub fn get_logical_device(&self) -> &Arc<Device> {
        &self.logical_device
    }

    pub fn clone_logical_device(&self) -> Arc<Device> {
        self.logical_device.clone()
    }

    pub fn get_queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    pub fn clone_queue(&self) -> Arc<Queue> {
        self.queue.clone()
    }
}
