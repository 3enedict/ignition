use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

use crate::renderer::core::logical_device::VglLogicalDevice;


use crate::object::vertex::Vertex;
use crate::object::VglObject;

use crate::DEBUG;

impl VglObject {
    pub fn rectangle(
        logical_device: &VglLogicalDevice,
        vertices: &Vec<Vertex>,
        indices: &Vec<u16>,
    ) -> Self {
        Self::check_rectangle_paramaters(vertices, indices);

        let vertex_buffer = Some(CpuAccessibleBuffer::from_iter(
            logical_device.clone_logical_device(),
            BufferUsage::all(),
            false,
            vertices.iter().cloned(),
        ).unwrap());

        let index_buffer = Some(CpuAccessibleBuffer::from_iter(
            logical_device.clone_logical_device(),
            BufferUsage::index_buffer(),
            false,
            indices.iter().cloned(),
        ).unwrap());

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn check_rectangle_paramaters(
        vertices: &Vec<Vertex>,
        indices: &Vec<u16>,
    ) {
        if DEBUG { 
            if vertices.len() % 4 != 0 { 
                panic!("Supplied rectangles don't have 4 vertices each (help: check vertices passed during rectangle creation)") 
            }

            if indices.len() % 6 != 0 { 
                panic!("Supplied rectangles don't have 6 indices each (help: check indexes passed during rectangle creation)") 
            }

            for vertex in vertices {
                if vertex.position[0] < -1.0 || vertex.position[0] > 1.0   ||   vertex.position[1] < -1.0 || vertex.position[1] > 1.0 {
                    panic!("Position out of bounds");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::vertex::Vertex;
    use crate::object::VglObject;

    #[test]
    #[should_panic]
    fn vertices_not_multiple_of_four_panics_in_debug_mode() {
        let vertices = vec!
            [
                Vertex { position: [ 0.0, -0.5] },
                Vertex { position: [ 0.5,  0.5] },
                Vertex { position: [-0.5,  0.5] },
            ];

        let indices = vec![0,0,0,0,0,0];

        VglObject::check_rectangle_paramaters(&vertices, &indices);
    }

    #[test]
    #[should_panic]
    fn indices_not_multiple_of_six_panics_in_debug_mode() {
        let vertices = vec!
            [
                Vertex{ position: [-0.5, -0.5] },
                Vertex{ position: [ 0.5, -0.5] },
                Vertex{ position: [ 0.5,  0.5] },
                Vertex{ position: [-0.5,  0.5] },
            ];

        let indices = vec![1,2,3,4,5,6, 7];

        VglObject::check_rectangle_paramaters(&vertices, &indices);
    }

    #[test]
    fn normal_inputs_do_not_panic() {
        let vertices = vec!
            [
                Vertex{ position: [-0.5, -0.5] },
                Vertex{ position: [ 0.5, -0.5] },
                Vertex{ position: [ 0.5,  0.5] },
                Vertex{ position: [-0.5,  0.5] },
            ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        VglObject::check_rectangle_paramaters(&vertices, &indices);
    }

    #[test]
    #[should_panic]
    fn vertices_out_of_range_panics_in_debug_mode() {
        let vertices = vec!
            [
                Vertex{ position: [-0.5, -0.5] },
                Vertex{ position: [ 0.5, -0.5] },
                Vertex{ position: [ 0.5,  0.5] },
                Vertex{ position: [-1.5,  0.5] },
            ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        VglObject::check_rectangle_paramaters(&vertices, &indices);
    }
}
