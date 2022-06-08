use log::info;
use std::any::TypeId;

use crate::ecs::IgnitionScene;

pub mod component_pool;
use component_pool::ComponentPool;

pub mod component_pool_trait;

impl IgnitionScene {
    pub fn with_component<G: 'static + std::fmt::Debug>(&mut self, component: G) -> &mut Self {
        let current_entity = self.available_entities[self.available_entities.len() - 1];
        info!(
            "Adding component to current entity under construction ({})",
            current_entity
        );

        self.component(current_entity, component);

        self
    }

    pub fn component<G: 'static + std::fmt::Debug>(&mut self, entity: usize, component: G) {
        info!(
            "Assigning component ({:?}) to entity ({})",
            component, entity
        );

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
