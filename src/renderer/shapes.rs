use wgpu::{Buffer, RenderPass, RenderPipeline, ShaderModuleDescriptor, TextureFormat};

use crate::renderer::core::{gpu::GPU, vertex_buffer::Vertex};

pub mod crackers;
pub mod doritos;

pub struct Shape {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,

    pub num_vertices: u32,
}

impl GPU {
    pub fn shape(
        &mut self,
        vertices: &Vec<Vertex>,
        shaders: &ShaderModuleDescriptor,
        format: TextureFormat,
    ) -> Shape {
        Shape {
            pipeline: self.ignite_pipeline(shaders, format),
            vertex_buffer: self.ignite_vertex_buffer(vertices),

            num_vertices: vertices.len() as u32,
        }
    }
}

impl Shape {
    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.draw(0..self.num_vertices, 0..1);
    }
}

/*
pub struct IndexedShape {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,

    pub num_indices: u32,
}

impl Engine {
    pub fn indexed_shape(
        &mut self,
        vertices: &Vec<Vertex>,
        indices: &Vec<u16>,
        shaders: &ShaderModuleDescriptor,
    ) -> IndexedShape {
        IndexedShape {
            pipeline: ignite_pipeline(self, shaders),
            vertex_buffer: ignite_vertex_buffer(self, vertices),
            index_buffer: ignite_index_buffer(self, indices),

            num_indices: indices.len() as u32,
        }
    }
}

impl IndexedShape {
    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), Uint16);

        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

*/
