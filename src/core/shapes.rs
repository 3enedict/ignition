use wgpu::{
    ShaderModuleDescriptor,

    RenderPass,
    RenderPipeline,
    Buffer,
};

use crate::core::{
    Engine,
    rendering::{
        vertex_buffer::{Vertex, ignite_vertex_buffer},
        pipeline::ignite_pipeline,
    },
};

pub mod triangle;

pub struct Shape {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,

    pub vertex_len: u32
}

pub fn shape(engine: &mut Engine, vertices: &Vec<Vertex>, shaders: ShaderModuleDescriptor) -> Shape {
    Shape {
        pipeline: ignite_pipeline(engine, shaders),
        vertex_buffer: ignite_vertex_buffer(engine, vertices),

        vertex_len: vertices.len() as u32,
    }
}

impl Shape {
    pub fn render<'a: 'b, 'b>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.draw(0..self.vertex_len, 0..1);
    }
}
