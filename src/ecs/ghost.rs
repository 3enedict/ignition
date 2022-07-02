use crate::ecs::{gizmos::PoolToolbox, ComponentPool, Scene};

impl Scene {
    pub fn toggle<G: 'static>(&mut self, entity: usize) {
        self.get_trait_mut::<G>().toggle_entity(entity);
    }

    pub fn enable<G: 'static>(&mut self, entity: usize) {
        self.get_trait_mut::<G>().enable_entity(entity);
    }

    pub fn disable<G: 'static>(&mut self, entity: usize) {
        self.get_trait_mut::<G>().disable_entity(entity);
    }
}

pub trait ComponentToggler {
    fn toggle_entity(&mut self, entity: usize);
    fn enable_entity(&mut self, entity: usize);
    fn disable_entity(&mut self, entity: usize);
    fn entity_in_scope(&mut self, entity: usize) -> bool;
    fn move_to_back(&mut self, entity: usize);
}

impl<G: 'static> ComponentToggler for ComponentPool<G> {
    fn toggle_entity(&mut self, entity: usize) {
        if self.entity_in_scope(entity) {
            self.disable_entity(entity);
        } else {
            self.enable_entity(entity);
        }
    }

    fn disable_entity(&mut self, entity: usize) {
        self.move_to_back(entity);
        self.num_components -= 1;
    }

    fn enable_entity(&mut self, entity: usize) {
        self.num_components += 1;
        self.move_to_back(entity);
    }

    /* Utility functions */

    fn entity_in_scope(&mut self, entity: usize) -> bool {
        (self.sparse_array[entity] as usize) < self.num_components
    }

    fn move_to_back(&mut self, entity: usize) {
        let component = self.sparse_array[entity] as usize;
        let component_destination = self.num_components - 1;

        let entity_destination = self.packed_array[component_destination];

        self.swap(entity, entity_destination, component, component_destination);
    }
}
