use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily};
use vulkano::device::DeviceExtensions;

use crate::renderer::setup::VglInstance;
use crate::renderer::setup::VglSurface;

// Note: The physical device is stored as an index to avoid lifetime problems within classes
//       The same is true for the instance.
pub struct VglPhysicalDevice {
    physical_device_index: usize,
    queue_family_id: u32,

    instance: Arc<Instance>,
}

impl VglPhysicalDevice {
    pub fn new(
        instance: &VglInstance,
        surface: &VglSurface,
        device_extensions: &DeviceExtensions,
    ) -> Self {
        let (physical_device, queue_family) = PhysicalDevice::enumerate(instance.get_instance())
            .filter(|&p| {
                p.supported_extensions().is_superset_of(device_extensions)
            })
        .filter_map(|p| {
            p.queue_families()
                .find(|&q| {
                    q.supports_graphics() && surface.get_surface().is_supported(q).unwrap_or(false)
                })
            .map(|q| (p, q))
        })
        .min_by_key(|(p, _)| {
            match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
            }
        })
        .unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        Self {
            physical_device_index: physical_device.index(),
            queue_family_id: queue_family.id(),

            instance: instance.clone_instance(),
        }
    }

    pub fn get_physical_device(&self) -> PhysicalDevice {
        PhysicalDevice::from_index(&self.instance, self.physical_device_index).unwrap()
    }

    pub fn get_queue_family(&self) -> QueueFamily {
        self.get_physical_device()
            .queue_family_by_id(self.queue_family_id).unwrap()
    }
}
