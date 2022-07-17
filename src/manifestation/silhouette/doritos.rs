use wgpu::{Buffer, RenderPass, RenderPipeline, ShaderModuleDescriptor};

use crate::manifestation::{apex::VertexGroup, silhouette::Renderable, Renderer};

#[derive(Debug)]
pub struct Doritos {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,

    pub num_vertices: u32,
}

impl Renderer {
    pub fn doritos(
        &mut self,
        vertex_group: &VertexGroup,
        shaders: ShaderModuleDescriptor,
    ) -> Box<dyn Renderable> {
        let doritos = Doritos {
            pipeline: self.pipeline(shaders),
            vertex_buffer: self.vertex_buffer(vertex_group),

            num_vertices: 0,
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
