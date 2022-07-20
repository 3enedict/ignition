use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages,
};

use crate::manifestation::{apex::VertexGroup, Renderer};

impl Renderer {
    pub fn vertex_buffer(&mut self, vertex_group: &VertexGroup) -> Buffer {
        let vertex_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &vertex_group.get(),
            usage: BufferUsages::VERTEX,
        });

        vertex_buffer
    }

    pub fn index_buffer(&mut self, indices: &Vec<u16>) -> Buffer {
        let index_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        index_buffer
    }
}
