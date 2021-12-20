use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{PrimaryAutoCommandBuffer, AutoCommandBufferBuilder};
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;

use crate::renderer::core::logical_device::VglLogicalDevice;

use crate::objects::vertex::Vertex;

pub struct VglTriangle {
    vertices: Option<[Vertex; 3]>,

    vertex_buffer: Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,
    index: u32,
}

impl VglTriangle {
    pub fn new(
        vertices: [Vertex; 3],
    ) -> Self {
        Self {
            vertices: Some(vertices),

            vertex_buffer: None,
            index: 0,
        }
    }

    pub fn setup(
        &mut self,
        logical_device: &VglLogicalDevice,
        index: u32,
    ) {
        self.vertex_buffer = Some(CpuAccessibleBuffer::from_iter(
            logical_device.clone_logical_device(),
            BufferUsage::all(),
            false,
            self.vertices.unwrap().iter().cloned(),
        ).unwrap());

        self.index = index;

        self.vertices = None;
    }

    pub fn render(
        &self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandPoolBuilder>,
    ) {
        builder
            .bind_vertex_buffers(self.index, self.get_vertex_buffer())
            .draw(self.get_vertex_buffer_len() as u32, 1, 0, 0)
            .unwrap();
    }

    fn get_vertex_buffer(&self) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        self.vertex_buffer.clone().unwrap()
    }

    fn get_vertex_buffer_len(&self) -> u64 {
        self.get_vertex_buffer().len()
    }
}
