use wgpu::ShaderModuleDescriptor;

use crate::core::{
    Engine,
    rendering::vertex_buffer::Vertex,
    shapes::{shape, Shape},
};

pub fn doritos(engine: &mut Engine, vertices: &Vec<Vertex>, shaders: ShaderModuleDescriptor) -> Shape {
    shape(engine, vertices, shaders)
}
