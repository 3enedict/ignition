use crate::core::objects::vertex::Vertex;
use crate::core::objects::VglObject;

impl VglObject {
    pub fn triangle(
        vertices: &Vec<Vertex>,
    ) -> Self {
        Self::check_triangle_parameters(vertices);

        Self {
            vertices: Some(vertices.clone()),
            indices: None,

            pipeline_id: 0,
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
