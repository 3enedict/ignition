use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferDescriptor, BufferUsages,
};

use crate::{manifestation::Renderer, Engine};

impl<R: Renderer> Engine<R> {
    pub fn buffer(&mut self, descriptor: BufferDescriptor) -> Buffer {
        self.renderer.device().create_buffer(&descriptor)
    }

    pub fn initialized_buffer(&mut self, descriptor: BufferInitDescriptor) -> Buffer {
        self.renderer.device().create_buffer_init(&descriptor)
    }

    pub fn vertex_buffer(&mut self, vertices: Vec<f32>) -> Buffer {
        let mut contents: Vec<u8> = Vec::with_capacity(vertices.len() * std::mem::size_of::<f32>());
        for value in vertices.into_iter() {
            contents.append(&mut value.to_le_bytes().to_vec());
        }

        self.initialized_buffer(BufferInitDescriptor {
            label: None,
            contents: &contents,
            usage: BufferUsages::VERTEX,
        })
    }
}
