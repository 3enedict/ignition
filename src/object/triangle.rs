use crate::renderer::core::logical_device::VglLogicalDevice;


use crate::object::vertex::Vertex;
use crate::object::VglObject;

use crate::DEBUG;

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
        if DEBUG { if vertices.len() % 3 != 0 { panic!("Supplied triangles don't have 3 vertices each") } }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::vertex::Vertex;
    use crate::object::VglObject;

    use crate::DEBUG;

    #[test]
    fn vertices_not_multiple_of_three_panics_in_debug_mode() {
        let vertices = vec!
            [
                Vertex { position: [ 0.0, -0.5] },
                Vertex { position: [ 0.5,  0.5] },
                Vertex { position: [-0.5,  0.5] },
                Vertex { position: [ 0.5,  0.5] },
            ];

        let result = std::panic::catch_unwind(|| VglObject::check_triangle_parameters(&vertices));

        assert_eq!(result.is_err(), DEBUG)
    }

    #[test]
    fn vertices_multiple_of_three_does_not_panic() {
        let vertices = vec!
            [
                Vertex { position: [ 0.0, -0.5] },
                Vertex { position: [ 0.5,  0.5] },
                Vertex { position: [-0.5,  0.5] },
            ];

        VglObject::check_triangle_parameters(&vertices);
    }
}
