use crate::ecs::{component::ComponentPool, Scene};
use std::any::TypeId;

impl Scene {
    pub fn get_component_pool<G: 'static>(&self) -> &ComponentPool<G> {
        self.component_pools
            .get(*self.component_indices.get(&TypeId::of::<G>()).unwrap())
            .unwrap()
            .as_any()
            .downcast_ref::<ComponentPool<G>>()
            .unwrap()
    }

    pub fn get_component_pool_mut<G: 'static>(&mut self) -> &mut ComponentPool<G> {
        self.component_pools
            .get_mut(*self.component_indices.get(&TypeId::of::<G>()).unwrap())
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentPool<G>>()
            .unwrap()
    }

    pub fn get_component<G: 'static>(&self, entity: usize) -> &G {
        self.get_component_pool::<G>()
            .component_array
            .get(entity)
            .unwrap()
    }

    pub fn get_component_mut<G: 'static>(&mut self, entity: usize) -> &G {
        self.get_component_pool_mut::<G>()
            .component_array
            .get(entity)
            .unwrap()
    }
}
