use wgpu::ShaderModuleDescriptor;

use crate::renderer::{core::vertex_buffer::Vertex, shapes::Renderable, Renderer};

impl Renderer {
    pub fn doritos(
        &mut self,
        vertices: &Vec<Vertex>,
        shaders: &ShaderModuleDescriptor,
    ) -> Box<dyn Renderable> {
        Box::new(self.shape(vertices, shaders))
    }
}
