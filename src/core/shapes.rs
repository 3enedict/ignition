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

pub struct IgnitionShapes { 
    pub pipelines: Vec<RenderPipeline>,
    pub vertex_buffers: Vec<Buffer>,
}

impl IgnitionShapes {
    pub fn new() -> Self {
        Self {
            pipelines: Vec::new(),
            vertex_buffers: Vec::new(),
        }
    }
}



pub fn ignite_shape(engine: &mut Engine, vertices: &Vec<Vertex>, shaders: ShaderModuleDescriptor) {
    let pipeline = ignite_pipeline(engine, shaders);
    let vertex_buffer = ignite_vertex_buffer(engine, vertices);

    engine.shapes.pipelines.push(pipeline);
    engine.shapes.vertex_buffers.push(vertex_buffer);
}
