use crate::core::logical_device::VglLogicalDevice;

use crate::objects::vertex::Vertex;
use crate::objects::VglObject;

impl VglObject {
    pub fn triangle(
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
    ) -> Self {
        Self::check_triangle_parameters(vertices);

        Self {
            vertex_buffer: Self::generate_vertex_buffer(logical_device, vertices),
            index_buffer: None,
        }
    }

    pub fn check_triangle_parameters(
        vertices: &Vec<Vertex>,
    ) {
        let expected_number_of_vertices_for_a_triangle: usize = 3;

        Self::check_vertices(vertices);
        Self::check_number_of_vertices(vertices, expected_number_of_vertices_for_a_triangle);
    }
}
