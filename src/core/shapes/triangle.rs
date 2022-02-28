use wgpu::{
    ShaderModuleDescriptor,

    RenderPipeline,
    Buffer,
};

use crate::core::Engine;
use crate::core::rendering::{
    vertex_buffer::{Vertex, ignite_vertex_buffer},
    pipeline::ignite_pipeline,
};

pub struct Triangle { 
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,

    pub vertex_len: u32
}

impl Triangle {
    pub fn ignite(engine: &mut Engine, vertices: &Vec<Vertex>, shaders: ShaderModuleDescriptor) -> Self {
        Self {
            pipeline: ignite_pipeline(engine, shaders),
            vertex_buffer: ignite_vertex_buffer(engine, vertices),

            vertex_len: vertices.len() as u32,
        }
    }
}
