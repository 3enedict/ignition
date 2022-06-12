use wgpu::ShaderModuleDescriptor;

use crate::{core::rendering::vertex_buffer::Vertex, core::Engine};

impl Engine {
    pub fn with_component<G: 'static>(&mut self, component: G) -> &mut Self {
        self.scene
            .component(self.scene.get_current_entity(), component);

        self
    }

    pub fn entity(&mut self) -> usize {
        self.scene.entity()
    }

    pub fn doritos(&mut self) -> usize {
        let entity = self.scene.get_current_entity();
        let vertices = self
            .scene
            .get_component_pool::<Vec<Vertex>>()
            .component_array
            .get(entity)
            .unwrap();

        let shader = self
            .scene
            .get_component_pool::<ShaderModuleDescriptor>()
            .component_array
            .get(entity)
            .unwrap();

        let doritos = self.gpu.shape(vertices, shader, self.window.config.format);
        self.scene.component(entity, doritos);

        self.scene.entity()
    }
}
