use wgpu::ShaderModuleDescriptor;

use crate::core::{
    rendering::vertex_buffer::Vertex,
    shapes::{shape, Shape},
    Engine,
};

pub fn doritos(
    engine: &mut Engine,
    vertices: &Vec<Vertex>,
    shaders: ShaderModuleDescriptor,
) -> Shape {
    shape(engine, vertices, shaders)
}
