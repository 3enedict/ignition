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
        // Note: Probably replaceable with .map()
        let mut contents: Vec<u8> = Vec::new();
        for value in vertices.into_iter() {
            contents.append(&mut bincode::serialize(&value).unwrap());
        }

        self.initialized_buffer(BufferInitDescriptor {
            label: None,
            contents: &contents,
            usage: BufferUsages::VERTEX,
        })
    }
}
