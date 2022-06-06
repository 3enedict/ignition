use std::any::TypeId;

use crate::ecs::IgnitionScene;

pub mod component_pool;
use component_pool::ComponentPool;

pub mod component_pool_trait;

pub mod component_builder;
use component_builder::ComponentBuilder;

impl IgnitionScene {
    // This is only really for aesthetics
    pub fn with_component<G: 'static>(&mut self, component: G) -> ComponentBuilder {
        let entity = self.entity();
        self.component(entity, component);

        ComponentBuilder {
            scene: self,
            entity,
        }
    }

    pub fn component<G: 'static>(&mut self, entity: usize, component: G) {
        if self.component_exists::<G>() {
            self.assign_component_to_entity(entity, component)
        } else {
            self.create_new_component_pool_with_entity(entity, component);
        }
    }

    /* Utility functions */

    pub fn get_component_pool<G: 'static>(&mut self) -> &mut ComponentPool<G> {
        self.component_pools
            .get_mut(*self.component_indices.get(&TypeId::of::<G>()).unwrap())
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentPool<G>>()
            .unwrap()
    }

    pub fn component_exists<G: 'static>(&mut self) -> bool {
        self.component_indices.contains_key(&TypeId::of::<G>())
    }

    pub fn assign_component_to_entity<G: 'static>(&mut self, entity: usize, component: G) {
        self.get_component_pool::<G>()
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
}
