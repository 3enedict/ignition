use crate::core::rendering::vertex_buffer::Vertex;
use crate::core::shapes::Shape;

pub struct Vertices {
    pub vertices: Vec<Vertex>,
}

pub struct IgnitionScene {
    pub vertices_component: Vec<Option<Vertices>>,
    pub shape_component: Vec<Option<Shape>>,
    pub render_component: Vec<Option<bool>>,
}

impl IgnitionScene {
    pub fn new() -> Self {
        Self {
            vertices_component: Vec::new(),
            shape_component: Vec::new(),
            render_component: Vec::new(),
        }
    }

    pub fn new_entity(
        &mut self,
        vertices: Option<Vertices>,
        shape: Option<Shape>,
        render: Option<bool>,
    ) {
        self.vertices_component.push(vertices);
        self.shape_component.push(shape);
        self.render_component.push(render);
    }
}
