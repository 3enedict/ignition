use crate::renderer::VglRenderer;

use crate::object::VglObject;
use crate::object::vertex::Vertex;

impl VglRenderer {
    pub fn add_triangles(
        &mut self,
        vertices: &mut Vec<Vertex>,
    ) {
        let triangle = VglObject::triangle(&self.logical_device, vertices);
        self.objects.push(triangle);
    }


    pub fn add_rectangles(
        &mut self,
        vertices: &mut Vec<Vertex>,
    ) {
        let rectangle = VglObject::rectangle(&self.logical_device, vertices);
        self.objects.push(rectangle);
    }
}
