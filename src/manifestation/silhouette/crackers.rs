/*
use wgpu::ShaderModuleDescriptor;
use crate::core::{rendering::vertex_buffer::Vertex, shapes::IndexedShape, Engine};
pub fn crackers(
    engine: &mut Engine,
    vertices: &Vec<Vertex>,
    shaders: &ShaderModuleDescriptor,
) -> IndexedShape {
    let (generated_vertices, generated_indices) = generate_rectangle(vertices);
    engine.indexed_shape(&generated_vertices, &generated_indices, shaders)
}
pub fn generate_rectangle(vertices: &Vec<Vertex>) -> (Vec<Vertex>, Vec<u16>) {
    let mut generated_vertices: Vec<Vertex> = Vec::new();
    let mut generated_indices: Vec<u16> = Vec::new();
    let mut index = 0;
    while index < vertices.len() {
        generated_vertices.append(&mut generate_rectangle_vertices(vertices, index));
        generated_indices.append(&mut generate_rectangle_indices(index));
        index = index + 2; //Skip vertex since they come in pairs
    }
    (generated_vertices, generated_indices)
}
/*
 *
 *  A                    *B*
 *   ---------------------
 *   |                   |
 *   |                   |
 *   |                   |
 *   ---------------------
 * *D*                    C
 *
 * (points in ** are auto generated)
 *
 */
pub fn generate_rectangle_vertices(vertices: &Vec<Vertex>, index: usize) -> Vec<Vertex> {
    let a = vertices[index];
    let c = vertices[index + 1];
    let b = Vertex {
        position: [c.position[0], a.position[1], 0.0],
        color: c.color,
    };
    let d = Vertex {
        position: [a.position[0], c.position[1], 0.0],
        color: a.color,
    };
    vec![a, c, b, d]
}
//                                                                                (0, 1, *2*, *3*)
pub fn generate_rectangle_indices(index: usize) -> Vec<u16> {
    // Awaited format: (A, C, *B*, *D*)
    let mut rectangle_layout = vec![0, 1, 2, 2, 3, 0];
    for rectangle_index in rectangle_layout.iter_mut() {
        *rectangle_index = *rectangle_index + index as u16 * 2;
    }
    rectangle_layout
}



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
