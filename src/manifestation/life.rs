use serde::Serialize;
use wgpu::{ShaderModuleDescriptor, VertexFormat};

use crate::{manifestation::apex::VertexGroup, Engine};

impl Engine {
    pub fn component<G: 'static>(&mut self, component: G) -> &mut Self {
        let entity = self.scene.get_current_entity();
        self.scene.component(entity, component);

        self
    }

    pub fn data<G: Serialize + std::fmt::Debug, const N: usize>(
        &mut self,
        data: [G; N],
        step: usize,
        format: VertexFormat,
    ) -> &mut Self {
        let entity = self.scene.get_current_entity();

        if self.scene.component_exists::<VertexGroup>(entity) {
            self.scene
                .get_component_mut::<VertexGroup>(entity)
                .unwrap()
                .data(data, step, format);
        } else {
            let mut vertex_group = VertexGroup::new();
            vertex_group.data(data, step, format);

            self.scene.component(entity, vertex_group);
        }

        self
    }

    pub fn xy<const N: usize>(&mut self, data: [f32; N]) -> &mut Self {
        self.data(data, 2, VertexFormat::Float32x2)
    }

    pub fn xyz<const N: usize>(&mut self, data: [f32; N]) -> &mut Self {
        self.data(data, 3, VertexFormat::Float32x3)
    }

    pub fn rgb<const N: usize>(&mut self, data: [f32; N]) -> &mut Self {
        self.data(data, 3, VertexFormat::Float32x3)
    }

    pub fn entity(&mut self) -> usize {
        self.scene.entity()
    }

    pub fn doritos(&mut self) -> usize {
        let entity = self.scene.get_current_entity();

        let shaders = unwrap_or!(
            self.scene.take_component::<ShaderModuleDescriptor>(entity),
            self.scene.entity()
        );

        let vertex_group = unwrap_or!(
            self.scene.get_component::<VertexGroup>(entity),
            self.scene.entity()
        );

        let doritos = self.renderer.doritos(vertex_group, shaders);
        self.scene.component(entity, doritos);

        return self.scene.entity();
    }
}
