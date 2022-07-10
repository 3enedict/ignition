use wgpu::ShaderModuleDescriptor;

use crate::renderer::{core::vertex_buffer::Vertex, shapes::Renderable, Renderer};

impl Renderer {
    pub fn doritos<G: Vertex + bytemuck::Pod>(
        &mut self,
        vertices: &Vec<G>,
        shaders: &ShaderModuleDescriptor,
    ) -> Box<dyn Renderable> {
        Box::new(self.shape(vertices, shaders))
    }
}
