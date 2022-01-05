use crate::renderer::core::logical_device::VglLogicalDevice;

use crate::object::vertex::Vertex;
use crate::object::VglObject;

impl VglObject {
    pub fn rectangle(
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
    ) -> Self {
        let (rectangle_vertices, rectangle_indices) = Self::generate_rectangle(vertices);

        Self {
            vertex_buffer: Self::generate_vertex_buffer(logical_device, &rectangle_vertices),
            index_buffer: Self::generate_index_buffer(logical_device, &rectangle_indices),
        }
    }

    pub fn generate_rectangle(
        vertices: &Vec<Vertex>,
    ) -> (Vec<Vertex>, Vec<u16>) {
        Self::check_rectangle_parameters(vertices);

        let mut rectangle_vertices = Vec::new();
        let mut rectangle_indices = Vec::new();

        for i in 0..vertices.len()/2 {
            Self::generate_rectangle_vertices(vertices, &mut rectangle_vertices, i);

            Self::generate_quadrilateral_indices(&mut rectangle_indices, i)
        }

        (rectangle_vertices, rectangle_indices)
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










    pub fn check_rectangle_parameters(
        vertices: &Vec<Vertex>,
    ) {
        let expected_number_of_vertices_for_a_rectangle: usize = 2;

        Self::check_vertices(vertices);
        Self::check_number_of_vertices(vertices, expected_number_of_vertices_for_a_rectangle);
    }

}

#[cfg(test)]
mod tests {
    use crate::object::vertex::Vertex;
    use crate::object::VglObject;

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
