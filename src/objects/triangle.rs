use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

use crate::renderer::core::logical_device::VglLogicalDevice;


use crate::objects::vertex::Vertex;

pub struct VglTriangle {
    pub vertices: Vec<Vertex>,

    vertex_buffer: Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,
}

impl VglTriangle {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),

            vertex_buffer: None,
        }
    }

    pub fn add_triangles(
        &mut self,
        vertices: &Vec<Vertex>,
    ) {
        self.vertices.extend(vertices.iter().cloned());
    }

    pub fn generate_vertex_buffer(
        &mut self,
        logical_device: &VglLogicalDevice,
    ) {
        self.vertex_buffer = Some(CpuAccessibleBuffer::from_iter(
                logical_device.clone_logical_device(),
                BufferUsage::all(),
                false,
                self.vertices.iter().cloned(),
        ).unwrap());
    }

    pub fn get_vertex_buffer(
        &self,
    ) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        self.vertex_buffer.clone().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::vertex::Vertex;
    use crate::objects::triangle::VglTriangle;

    fn compare_vecs<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn add_triangles_works() {
        let triangles = vec!
            [
            Vertex { position: [ 0.55, -0.5 ] },
            Vertex { position: [ 0.55,  0.55] },
            Vertex { position: [-0.5 ,  0.55] },

            Vertex { position: [-0.55,  0.5 ] },
            Vertex { position: [-0.55, -0.55] },
            Vertex { position: [ 0.5 , -0.55] },
            ];

        let mut triangle = VglTriangle::new();

        triangle.add_triangles(&triangles);

        assert!(compare_vecs(&triangle.vertices, &triangles))
    }
}
