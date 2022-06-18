use wgpu::{ShaderModuleDescriptor, TextureFormat};

use crate::renderer::{
    core::{gpu::GPU, vertex_buffer::Vertex},
    shapes::Shape,
};

impl GPU {
    pub fn doritos(
        &mut self,
        vertices: &Vec<Vertex>,
        shaders: &ShaderModuleDescriptor,
        format: TextureFormat,
    ) -> Shape {
        self.shape(vertices, shaders, format)
    }
}
