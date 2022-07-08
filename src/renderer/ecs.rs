use wgpu::ShaderModuleDescriptor;

use crate::{renderer::core::vertex_buffer::Vertex, Engine};

impl Engine {
    pub fn with_component<G: 'static>(&mut self, component: G) -> &mut Self {
        self.scene
            .component(self.scene.get_current_entity(), component);

        self
    }

    pub fn point(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> &mut Self {
        let entity = self.scene.get_current_entity();
        if self.scene.component_exists::<Vec<Vertex>>() {
            if self.scene.get::<Vec<Vertex>>().has_component(entity) {
                self.scene
                    .get_component_mut::<Vec<Vertex>>(entity)
                    .push(Vertex {
                        position: [x, y, z],
                        color: [r, g, b],
                    });

                return self;
            } else {
                self.with_component(vec![Vertex {
                    position: [x, y, z],
                    color: [r, g, b],
                }])
            }
        } else {
            self.with_component(vec![Vertex {
                position: [x, y, z],
                color: [r, g, b],
            }])
        }
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
