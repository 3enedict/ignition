use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

use crate::renderer::core::logical_device::VglLogicalDevice;


use crate::object::vertex::Vertex;
use crate::object::VglObject;

impl VglObject {
    pub fn triangle(
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
    ) -> Self {
        let vertex_buffer = Some(CpuAccessibleBuffer::from_iter(
                logical_device.clone_logical_device(),
                BufferUsage::all(),
                false,
                vertices.iter().cloned(),
        ).unwrap());

        Self {
            vertex_buffer,
            index_buffer: None,
        }
    }
}
