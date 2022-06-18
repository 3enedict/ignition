use std::any::TypeId;

use crate::ecs::Scene;

pub mod component_pool;
use component_pool::ComponentPool;

pub mod component_pool_trait;

impl Scene {
    pub fn component<G: 'static>(&mut self, entity: usize, component: G) {
        if self.component_exists::<G>() {
            self.assign_component_to_entity(entity, component)
        } else {
            self.create_new_component_pool_with_entity(entity, component);
        }
    }

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

    /* Utility functions */

    pub fn assign_component_to_entity<G: 'static>(&mut self, entity: usize, component: G) {
        self.get_component_pool_mut::<G>()
            .assign_component_to_entity(entity, component);
    }

    pub fn create_new_component_pool_with_entity<G: 'static>(
        &mut self,
        entity: usize,
        component: G,
    ) {
        self.component_indices
            .insert(TypeId::of::<G>(), self.component_pools.len());

        self.component_pools
            .push(Box::new(ComponentPool::new_with_entity(entity, component)));
    }

    pub fn component_exists<G: 'static>(&mut self) -> bool {
        self.component_indices.contains_key(&TypeId::of::<G>())
    }
}
