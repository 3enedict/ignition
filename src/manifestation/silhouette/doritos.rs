use wgpu::{Buffer, RenderPass, RenderPipeline, ShaderModuleDescriptor};

use crate::manifestation::{apex::Vertex, silhouette::Renderable, Renderer};

#[derive(Debug)]
pub struct Doritos {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,

    pub num_vertices: u32,
}

impl Renderer {
    pub fn doritos<G: Vertex + bytemuck::Pod>(
        &mut self,
        vertices: &Vec<G>,
        shaders: &ShaderModuleDescriptor,
    ) -> Box<dyn Renderable> {
        let doritos = Doritos {
            pipeline: self.pipeline::<G>(shaders),
            vertex_buffer: self.vertex_buffer(vertices),

            num_vertices: vertices.len() as u32,
        };

        Box::new(doritos)
    }
}

impl Renderable for Doritos {
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
