use std::any::TypeId;

use crate::ecs::Scene;

pub mod component_pool;
use component_pool::ComponentPool;

pub mod component_pool_trait;
pub mod getters;
pub mod toggle;

impl Scene {
    pub fn component<G: 'static>(&mut self, entity: usize, component: G) {
        if self.component_exists::<G>() {
            self.assign_component_to_entity(entity, component)
        } else {
            self.create_new_component_pool_with_entity(entity, component);
        }
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
