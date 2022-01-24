extern crate vgl;

use vgl::core::objects::VglObject;
use vgl::core::objects::vertex::Vertex;

pub struct Model {
    triangle: VglObject,
}

impl Model {
    pub fn new() -> Self {
        let triangle = VglObject::triangle(&vec!
            [
            Vertex { position: [ 0.0, -0.5] },
            Vertex { position: [ 0.5,  0.5] },
            Vertex { position: [-0.5,  0.5] },
            ]);

        Self { triangle }
    }

    pub fn get_object(&self) -> &VglObject {
        &self.triangle
    }
}
