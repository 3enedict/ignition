use wgpu::ShaderModuleDescriptor;

use crate::{renderer::core::vertex_buffer::Vertex, Engine};

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
        let doritos = self.renderer.shape(
            self.scene.get_component::<Vec<Vertex>>(entity),
            self.scene.get_component::<ShaderModuleDescriptor>(entity),
        );

        self.scene.component(entity, doritos);
        self.scene.entity()
    }
}
