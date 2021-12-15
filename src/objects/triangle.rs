use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

use crate::renderer::VglRenderer;

use crate::objects::vertex::Vertex;

pub struct VglTriangle {
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

impl VglTriangle {
    pub fn new(
        renderer: &VglRenderer,
        vertices: [Vertex; 3],
    ) -> Self {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            renderer.logical_device().clone_logical_device(),
            BufferUsage::all(),
            false,
            vertices.iter().cloned(),
        ).unwrap();

        Self {
            vertex_buffer,
        }
    }

    pub fn get_vertex_buffer(self) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        self.vertex_buffer
    }
}
