use wgpu::ShaderModuleDescriptor;

use crate::core::{
    rendering::vertex_buffer::Vertex,
    shapes::{indexed_shape, IndexedShape},
    Engine,
};

pub fn crackers(
    engine: &mut Engine,
    vertices: &Vec<Vertex>,
    shaders: ShaderModuleDescriptor,
) -> IndexedShape {
    let (generated_vertices, generated_indices) = generate_rectangle(vertices);

    indexed_shape(engine, &generated_vertices, &generated_indices, shaders)
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
