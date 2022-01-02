use std::sync::Arc;

use vulkano::buffer::CpuAccessibleBuffer;

use crate::renderer::core::logical_device::VglLogicalDevice;


use crate::object::vertex::Vertex;
use crate::object::VglObject;

use crate::DEBUG;

impl VglObject {
    pub fn rectangle(
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
    ) -> Self {
        let (rectangle_vertices, rectangle_indices) = Self::generate_rectangle(vertices);

        let (vertex_buffer, index_buffer) = Self::generate_rectangle_buffers(logical_device, &rectangle_vertices, &rectangle_indices);

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn generate_rectangle(
        vertices: &Vec<Vertex>,
    ) -> (Vec<Vertex>, Vec<u16>) {
        Self::check_rectangle_paramaters(vertices);

        let mut rectangle_vertices = Vec::new();
        let mut rectangle_indices = Vec::new();

        for i in 0..vertices.len()/2 {
            Self::generate_rectangle_vertices(vertices, &mut rectangle_vertices, i);

            Self::generate_rectangle_indices(&mut rectangle_indices, i)
        }

        (rectangle_vertices, rectangle_indices)
    }

    pub fn check_rectangle_paramaters(
        vertices: &Vec<Vertex>,
    ) {
        if DEBUG { 
            if vertices.len() % 2 != 0 {
                panic!("Supplied rectangles don't have 2 vertices each (help: check vertices passed during rectangle creation)")
            }

            for vertex in vertices {
                if vertex.position[0] < -1.0 || vertex.position[0] > 1.0   ||   vertex.position[1] < -1.0 || vertex.position[1] > 1.0 {
                    panic!("Position in rectangle out of bounds (help: make sure each vertex position does not exceed 1 or -1)");
                }
            }
        }
    }

    fn generate_rectangle_vertices(
        vertices: &Vec<Vertex>,
        rectangle_vertices: &mut Vec<Vertex>,
        increment: usize,
    ) {
        let vertex_increment = increment * 2;

        rectangle_vertices
            .extend(vec!
                    [
                        vertices[vertex_increment],
                        Vertex { position: [vertices[vertex_increment].position[0], vertices[vertex_increment + 1].position[1]] },
                        vertices[vertex_increment + 1],
                        Vertex { position: [vertices[vertex_increment + 1].position[0], vertices[vertex_increment].position[1]] },
                    ].iter().copied()
            );
    }
    fn generate_rectangle_indices(
        rectangle_indices: &mut Vec<u16>,
        increment: usize,
    ) {
        let index_increment = increment as u16 * 4;

        rectangle_indices
            .extend(vec!
                    [
                        0 + index_increment,
                        1 + index_increment,
                        2 + index_increment,
                        2 + index_increment,
                        3 + index_increment,
                        0 + index_increment,
                    ].iter().copied()
            );
    }

    pub fn generate_rectangle_buffers(
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
        indices: &Vec<u16>,
    ) -> (Option<Arc<CpuAccessibleBuffer<[Vertex]>>>, Option<Arc<CpuAccessibleBuffer<[u16]>>>) {
        (Self::generate_vertex_buffer(logical_device, vertices), Self::generate_index_buffer(logical_device, indices))
    }
}

#[cfg(test)]
mod tests {
    use crate::object::vertex::Vertex;
    use crate::object::VglObject;

    use crate::DEBUG;

    // Check input

    #[test]
    fn vertices_not_multiple_of_two_panics_in_debug_mode() {
        let vertices = vec!
            [
                Vertex { position: [ 0.0, -0.5] },
                Vertex { position: [ 0.5,  0.5] },
                Vertex { position: [-0.5,  0.5] },
            ];

        let result = std::panic::catch_unwind(|| VglObject::generate_rectangle(&vertices));

        assert_eq!(result.is_err(), DEBUG)
    }

    #[test]
    fn vertices_out_of_range_panics_in_debug_mode() {
        let vertices = vec!
            [
                Vertex{ position: [-1.5, -0.5] },
                Vertex{ position: [ 0.5, -0.5] },
            ];

        let result = std::panic::catch_unwind(|| VglObject::generate_rectangle(&vertices));

        assert_eq!(result.is_err(), DEBUG)
    }


    // Check the logic behind creating a rectangle from two points

    #[test]
    fn vertices_correctly_generated() {
        let original_vertices = vec!
            [
                Vertex{ position: [ 0.0,  0.0] },
                Vertex{ position: [ 0.5,  0.5] },
            ];

        let expected_vertices = vec!
            [
                Vertex{ position: [ 0.0,  0.0] },
                Vertex{ position: [ 0.0,  0.5] },
                Vertex{ position: [ 0.5,  0.5] },
                Vertex{ position: [ 0.5,  0.0] },
            ];

        let (generated_vertices, _indices) = VglObject::generate_rectangle(&original_vertices);

        assert_eq!(generated_vertices, expected_vertices);
    }

    #[test]
    fn vertices_correctly_generated_with_two_rectangles() {
        let original_vertices = vec!
            [
                Vertex{ position: [-0.25,  0.75] },
                Vertex{ position: [-0.75, -0.75] },

                Vertex{ position: [ 0.75,  0.75] },
                Vertex{ position: [ 0.25, -0.75] },
            ];

        let expected_vertices = vec!
            [
                Vertex{ position: [-0.25,  0.75] },
                Vertex{ position: [-0.25,  -0.75] },
                Vertex{ position: [-0.75, -0.75] },
                Vertex{ position: [-0.75,  0.75] },

                Vertex{ position: [ 0.75,  0.75] },
                Vertex{ position: [ 0.75, -0.75] },
                Vertex{ position: [ 0.25, -0.75] },
                Vertex{ position: [ 0.25,  0.75] },
            ];

        let (generated_vertices, _indices) = VglObject::generate_rectangle(&original_vertices);

        assert_eq!(generated_vertices, expected_vertices);
    }

    #[test]
    fn indices_correctly_generated() {
        let original_vertices = vec!
            [
                Vertex{ position: [ 0.0,  0.0] },
                Vertex{ position: [ 0.5,  0.5] },
            ];

        let expected_indices = vec![0, 1, 2, 2, 3, 0];

        let (_generated_vertices, generated_indices) = VglObject::generate_rectangle(&original_vertices);

        assert_eq!(generated_indices, expected_indices);
    }

    #[test]
    fn indices_correctly_generated_with_two_rectangles() {
        let original_vertices = vec!
            [
                Vertex{ position: [-0.75, -0.75] },
                Vertex{ position: [-0.25,  0.75] },

                Vertex{ position: [ 0.25, -0.75] },
                Vertex{ position: [ 0.75,  0.75] },
            ];

        let expected_indices = vec![0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4];

        let (_generated_vertices, generated_indices) = VglObject::generate_rectangle(&original_vertices);

        assert_eq!(generated_indices, expected_indices);
    }
}
