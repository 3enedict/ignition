use wgpu::{ShaderModuleDescriptor, TextureFormat};

use crate::core::{
    rendering::{gpu::IgnitionGPU, vertex_buffer::Vertex},
    shapes::Shape,
};

impl IgnitionGPU {
    pub fn doritos(
        &mut self,
        vertices: &Vec<Vertex>,
        shaders: &ShaderModuleDescriptor,
        format: TextureFormat,
    ) -> Shape {
        self.shape(vertices, shaders, format)
    }
}
