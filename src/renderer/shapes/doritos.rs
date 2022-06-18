use wgpu::ShaderModuleDescriptor;

use crate::renderer::{core::vertex_buffer::Vertex, shapes::Shape, Renderer};

impl Renderer {
    pub fn doritos(&mut self, vertices: &Vec<Vertex>, shaders: &ShaderModuleDescriptor) -> Shape {
        self.shape(vertices, shaders)
    }
}
