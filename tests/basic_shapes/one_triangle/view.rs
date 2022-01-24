extern crate vgl;

use vgl::core::VglRenderer;
use vgl::core::rendering::parameters::VglRendererParameters;

use vgl::core::objects::VglObject;

pub struct View {
    pub renderer: VglRenderer,
}

impl View {
    pub fn new() -> Self {
        Self { renderer: VglRenderer::new(VglRendererParameters::default()) }
    }

    pub fn setup(&mut self, triangle: &VglObject) {
        self.renderer.add_objects(triangle);
    }

    pub fn update(&mut self) {
        self.renderer.draw();
    }
}
