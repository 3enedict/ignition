use std::sync::Arc;

use vulkano::buffer::{TypedBufferAccess, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;


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

    pub fn get_vertex_buffer(&self) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        self.vertex_buffer.clone().unwrap()
    }

    pub fn get_index_buffer(&self) -> Arc<CpuAccessibleBuffer<[u16]>> {
        self.index_buffer.clone().unwrap()
    }
}
