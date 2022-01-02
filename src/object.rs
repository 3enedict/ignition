use std::sync::Arc;

use vulkano::buffer::{BufferUsage, TypedBufferAccess, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;


use crate::renderer::core::logical_device::VglLogicalDevice;


pub mod vertex;
use crate::object::vertex::Vertex;

pub mod triangle;
pub mod rectangle;

pub struct VglObject {
    vertex_buffer: Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,
    index_buffer: Option<Arc<CpuAccessibleBuffer<[u16]>>>
}

impl VglObject {
    pub fn draw(
        &mut self,
        command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandPoolBuilder>,
    ) {
        command_buffer_builder
            .bind_vertex_buffers(0, self.get_vertex_buffer());

        if self.index_buffer.is_some() {
            command_buffer_builder
                .bind_index_buffer(self.get_index_buffer())
                .draw_indexed(self.get_index_buffer().len() as u32, 1, 0, 0, 0)
                .unwrap();
        } else {
            command_buffer_builder
                .draw(self.get_vertex_buffer().len() as u32, 1, 0, 0)
                .unwrap();
        }
    }

    fn generate_vertex_buffer(
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
    ) -> Option<Arc<CpuAccessibleBuffer<[Vertex]>>> {
        Some(CpuAccessibleBuffer::from_iter(
            logical_device.clone_logical_device(),
            BufferUsage::all(),
            false,
            vertices.iter().cloned(),
        ).unwrap())
    }

    fn generate_index_buffer(
        logical_device: &VglLogicalDevice,
        indices: &Vec<u16>,
    ) -> Option<Arc<CpuAccessibleBuffer<[u16]>>> {
        Some(CpuAccessibleBuffer::from_iter(
            logical_device.clone_logical_device(),
            BufferUsage::index_buffer(),
            false,
            indices.iter().cloned(),
        ).unwrap())
    }

    pub fn get_vertex_buffer(&self) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        self.vertex_buffer.clone().unwrap()
    }

    pub fn get_index_buffer(&self) -> Arc<CpuAccessibleBuffer<[u16]>> {
        self.index_buffer.clone().unwrap()
    }
}
