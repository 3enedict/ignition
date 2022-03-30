use wgpu::{
    ShaderModuleDescriptor,

    RenderPass,
    RenderPipeline,
    Buffer,

    IndexFormat::Uint16, 
};

use crate::core::{
    Engine,
    rendering::{
        vertex_buffer::{Vertex, ignite_vertex_buffer},
        index_buffer::ignite_index_buffer,
        pipeline::ignite_pipeline,
    },
};

pub mod doritos;
pub mod crackers;

pub struct Shape {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,

    pub num_vertices: u32,
}

pub fn shape(engine: &mut Engine, vertices: &Vec<Vertex>, shaders: ShaderModuleDescriptor) -> Shape {
    Shape {
        pipeline: ignite_pipeline(engine, shaders),
        vertex_buffer: ignite_vertex_buffer(engine, vertices),

        num_vertices: vertices.len() as u32,
    }
}

impl Shape {
    pub fn render<'a: 'b, 'b>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.draw(0..self.num_vertices, 0..1);
    }
}











pub struct IndexedShape {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,

    pub num_indices: u32,
}

pub fn indexed_shape(engine: &mut Engine, vertices: &Vec<Vertex>, indices: &Vec<u16>, shaders: ShaderModuleDescriptor) -> IndexedShape {
    IndexedShape {
        pipeline: ignite_pipeline(engine, shaders),
        vertex_buffer: ignite_vertex_buffer(engine, vertices),
        index_buffer: ignite_index_buffer(engine, indices),

        num_indices: indices.len() as u32,
    }
}

impl IndexedShape {
    pub fn render<'a: 'b, 'b>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), Uint16);

        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
