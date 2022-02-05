use std::sync::Arc;

use vulkano::device::{Device, DeviceExtensions, Queue, Features};
use vulkano::device::physical::{PhysicalDevice, QueueFamily};


pub fn create_logical_device(
    device_extensions: &DeviceExtensions,
    physical_device: &PhysicalDevice,
    queue_family: &QueueFamily,
) -> (Arc<Device>, Arc<Queue>) {
    let (logical_device, mut queues) = Device::new(
        physical_device.clone(),
        &Features::none(),
        &physical_device
        .required_extensions()
        .union(&device_extensions),
        [(queue_family.clone(), 0.5)].iter().cloned(),
    )
        .unwrap();

    let queue = queues.next().unwrap();

    (logical_device, queue)
}
