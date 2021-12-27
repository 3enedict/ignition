use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;
use vulkano::buffer::TypedBufferAccess;


use crate::renderer::core::logical_device::VglLogicalDevice;


use crate::objects::vertex::Vertex;

pub struct VglRectangle {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,

    vertex_buffer: Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,
    index_buffer: Option<Arc<CpuAccessibleBuffer<[u16]>>>
}

impl VglRectangle {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),

            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn add(
        &mut self,
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
        indices: &Vec<u16>,
    ) {
        self.vertices.extend(vertices.iter().cloned());
        self.indices.extend(indices.iter().cloned());

        self.vertex_buffer = Some(CpuAccessibleBuffer::from_iter(
                logical_device.clone_logical_device(),
                BufferUsage::all(),
                false,
                self.vertices.iter().cloned(),
        ).unwrap());

        self.index_buffer = Some(CpuAccessibleBuffer::from_iter(
                logical_device.clone_logical_device(),
                BufferUsage::index_buffer(),
                false,
                self.indices.iter().cloned(),
        ).unwrap());
    }

    pub fn draw(
        &mut self,
        command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandPoolBuilder>,
    ) {
        if self.vertex_buffer.is_some() {
            command_buffer_builder
                .bind_vertex_buffers(0, self.get_vertex_buffer())
                .bind_index_buffer(self.get_index_buffer())
                .draw_indexed(self.get_index_buffer().len() as u32, 1, 0, 0, 0)
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
