use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

use crate::renderer::core::logical_device::VglLogicalDevice;


use crate::objects::vertex::Vertex;

pub struct VglTriangle {
    pub vertices: Vec<Vertex>,

    vertex_buffer: Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,
}

impl VglTriangle {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),

            vertex_buffer: None,
        }
    }

    pub fn add_triangles(
        &mut self,
        logical_device: &VglLogicalDevice,
        vertices: &mut Vec<Vertex>,
    ) {
        self.vertices.append(vertices);

        self.generate_vertex_buffer(logical_device);
    }

    fn generate_vertex_buffer(
        &mut self,
        logical_device: &VglLogicalDevice,
    ) {
        self.vertex_buffer = Some(CpuAccessibleBuffer::from_iter(
                logical_device.clone_logical_device(),
                BufferUsage::all(),
                false,
                self.vertices.iter().cloned(),
        ).unwrap());
    }

    pub fn get_vertex_buffer(
        &self,
    ) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        self.vertex_buffer.clone().unwrap()
    }
}
