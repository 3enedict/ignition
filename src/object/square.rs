use crate::renderer::core::logical_device::VglLogicalDevice;

use crate::object::vertex::Vertex;
use crate::object::VglObject;

use crate::DEBUG;

impl VglObject {
    pub fn square(
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
        sizes: &Vec<f32>,
    ) -> Self {
        let (square_vertices, square_indices) = Self::generate_square(vertices, sizes);

        Self {
            vertex_buffer: Self::generate_vertex_buffer(logical_device, &square_vertices),
            index_buffer: Self::generate_index_buffer(logical_device, &square_indices),
        }
    }





    pub fn generate_square(
        vertices: &Vec<Vertex>,
        sizes: &Vec<f32>,
    ) -> (Vec<Vertex>, Vec<u16>) {
        Self::check_square_parameters(vertices, sizes);

        let mut square_vertices = Vec::new();
        let mut square_indices = Vec::new();

        for i in 0..vertices.len() {
            Self::generate_square_vertices(&vertices[i], &mut square_vertices, &sizes[i]);

            Self::generate_quadrilateral_indices(&mut square_indices, i)
        }

        (square_vertices, square_indices)
    }

    pub fn generate_square_vertices(
        vertex: &Vertex,
        square_vertices: &mut Vec<Vertex>,
        size: &f32,
    ) {
        square_vertices
            .extend(vec!
                [
                Vertex { position: [vertex.position[0] - size, vertex.position[1] - size] },
                Vertex { position: [vertex.position[0] + size, vertex.position[1] - size] },
                Vertex { position: [vertex.position[0] + size, vertex.position[1] + size] },
                Vertex { position: [vertex.position[0] - size, vertex.position[1] + size] },
                ].iter().copied()
            );
    }






    pub fn check_square_parameters(
        vertices: &Vec<Vertex>,
        sizes: &Vec<f32>,
    ) {
        Self::check_vertices(vertices);
        Self::check_generated_square_not_out_of_bounds(vertices, sizes);
    }

    pub fn check_generated_square_not_out_of_bounds(
        vertices: &Vec<Vertex>,
        sizes: &Vec<f32>,
    ) {
        if DEBUG {
            for i in 0..vertices.len() {
                let vertex = vertices[i];
                let size = sizes[i];

                if vertex.position[0] + size > 1.0 || vertex.position[0] - size < -1.0 || vertex.position[1] + size > 1.0 || vertex.position[1] - size < -1.0 {
                    panic!("Square out of range. (help: Make sure that supplied sizes don't make the square go out of bounds)")
                }
            }
        }
    }


}

#[cfg(test)]
mod tests {
    use crate::object::vertex::Vertex;
    use crate::object::VglObject;

    use crate::DEBUG;

    #[test]
    fn square_out_of_bounds_to_the_north_panics_in_debug_mode() {
        let vertex = vec!
            [
            Vertex { position: [ 0.0, -0.5] },
            ];

        let size = vec![0.75];

        let result = std::panic::catch_unwind(|| VglObject::check_square_parameters(&vertex, &size));

        assert_eq!(result.is_err(), DEBUG)
    }

    #[test]
    fn square_out_of_bounds_to_the_south_panics_in_debug_mode() {
        let vertex = vec!
            [
            Vertex { position: [ 0.0,  0.5] },
            ];

        let size = vec![0.75];

        let result = std::panic::catch_unwind(|| VglObject::check_square_parameters(&vertex, &size));

        assert_eq!(result.is_err(), DEBUG)
    }

    #[test]
    fn square_out_of_bounds_to_the_east_panics_in_debug_mode() {
        let vertex = vec!
            [
            Vertex { position: [ 0.5,  0.0] },
            ];

        let size = vec![0.75];

        let result = std::panic::catch_unwind(|| VglObject::check_square_parameters(&vertex, &size));

        assert_eq!(result.is_err(), DEBUG)
    }

    #[test]
    fn square_out_of_bounds_to_the_west_panics_in_debug_mode() {
        let vertex = vec!
            [
            Vertex { position: [-0.5,  0.0] },
            ];

        let size = vec![0.75];

        let result = std::panic::catch_unwind(|| VglObject::check_square_parameters(&vertex, &size));

        assert_eq!(result.is_err(), DEBUG)
    }

    #[test]
    fn second_square_out_of_bounds_panics_in_debug_mode() {
        let vertices = vec!
            [
            Vertex { position: [ 0.0,  0.0] },
            Vertex { position: [-0.5,  0.0] },
            ];

        let sizes = vec![0.1, 0.75];

        let result = std::panic::catch_unwind(|| VglObject::check_square_parameters(&vertices, &sizes));

        assert_eq!(result.is_err(), DEBUG)
    }




    #[test]
    fn size_out_of_range_panics_in_debug_mode() {
        let vertices = vec!
            [
            Vertex { position: [ 0.0,  0.0] },
            ];

        let sizes = vec![1.1];

        let result = std::panic::catch_unwind(|| VglObject::check_square_parameters(&vertices, &sizes));

        assert_eq!(result.is_err(), DEBUG)
    }

    #[test]
    fn second_size_out_of_range_panics_in_debug_mode() {
        let vertices = vec!
            [
            Vertex { position: [ 0.0,  0.0] },
            Vertex { position: [ 0.0,  0.0] },
            ];

        let sizes = vec![0.1, 1.1];

        let result = std::panic::catch_unwind(|| VglObject::check_square_parameters(&vertices, &sizes));

        assert_eq!(result.is_err(), DEBUG)
    }














    #[test]
    fn vertices_generated_correctly() {
        let original_vertices = vec!
            [
            Vertex { position: [ 0.0,  0.0] },
            ];

        let sizes = vec![0.1];


        let expected_vertices = vec!
            [
            Vertex { position: [-0.1, -0.1] },
            Vertex { position: [ 0.1, -0.1] },
            Vertex { position: [ 0.1,  0.1] },
            Vertex { position: [-0.1,  0.1] },
            ];

        let (square_vertices, _square_indices) = VglObject::generate_square(&original_vertices, &sizes);

        assert_eq!(square_vertices, expected_vertices);
    }

    #[test]
    fn vertices_generated_correctly_with_two_squares() {
        let original_vertices = vec!
            [
            Vertex { position: [ 0.0,  0.0] },

            Vertex { position: [ 0.0,  0.0] },
            ];

        let sizes = vec![0.1, 0.3];


        let expected_vertices = vec!
            [
            Vertex { position: [-0.1, -0.1] },
            Vertex { position: [ 0.1, -0.1] },
            Vertex { position: [ 0.1,  0.1] },
            Vertex { position: [-0.1,  0.1] },

            Vertex { position: [-0.3, -0.3] },
            Vertex { position: [ 0.3, -0.3] },
            Vertex { position: [ 0.3,  0.3] },
            Vertex { position: [-0.3,  0.3] },
            ];

        let (square_vertices, _square_indices) = VglObject::generate_square(&original_vertices, &sizes);

        assert_eq!(square_vertices, expected_vertices);
    }




    #[test]
    fn indices_correctly_generated() {
        let original_vertices = vec!
            [
            Vertex{ position: [ 0.0,  0.0] },
            ];

        let sizes = vec![0.1];


        let expected_indices = vec![0, 1, 2, 2, 3, 0];

        let (_generated_vertices, generated_indices) = VglObject::generate_square(&original_vertices, &sizes);

        assert_eq!(generated_indices, expected_indices);
    }

    #[test]
    fn indices_correctly_generated_with_two_squares() {
        let original_vertices = vec!
            [
            Vertex { position: [ 0.0,  0.0] },

            Vertex { position: [ 0.2,  0.0] },
            ];

        let sizes = vec![0.1, 0.3];


        let expected_indices = vec![0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4];

        let (_generated_vertices, generated_indices) = VglObject::generate_square(&original_vertices, &sizes);

        assert_eq!(generated_indices, expected_indices);
    }
}
