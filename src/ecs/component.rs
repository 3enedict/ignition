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
            self.assign_component(entity, component)
        } else {
            self.new_component_pool(entity, component);
        }
    }

    pub fn component_exists<G: 'static>(&mut self) -> bool {
        self.component_indices.contains_key(&TypeId::of::<G>())
    }

    /* Utility functions */

    pub fn assign_component<G: 'static>(&mut self, entity: usize, component: G) {
        self.get_mut::<G>().assign_component(entity, component);
    }

    pub fn new_component_pool<G: 'static>(&mut self, entity: usize, component: G) {
        self.push_new_component_index::<G>();
        self.push_new_component_pool(entity, component);
    }

    pub fn push_new_component_index<G: 'static>(&mut self) {
        let type_id = TypeId::of::<G>();
        let index = self.component_pools.len();
        self.component_indices.insert(type_id, index);
    }

    pub fn push_new_component_pool<G: 'static>(&mut self, entity: usize, component: G) {
        let component_pool = Box::new(ComponentPool::new_with_entity(entity, component));
        self.component_pools.push(component_pool);
    }
}
