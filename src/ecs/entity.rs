use crate::ecs::IgnitionScene;

impl IgnitionScene {
    pub fn entity(&mut self) -> usize {
        let new_entity = self.generate_new_id();

        new_entity
    }

    pub fn delete(&mut self, entity: usize) {
        self.entity_count -= 1;
        self.available_entities.push(entity);
        self.delete_entity_from_each_component_pool(entity);
    }

    /* Utility functions */

    pub fn generate_new_id(&mut self) -> usize {
        self.available_entities.pop().unwrap_or_else(|| {
            let id = self.entity_count;
            self.entity_count += 1;

            self.push_empty_entity_to_each_component_pool();

            id
        })
    }

    pub fn push_empty_entity_to_each_component_pool(&mut self) {
        for component_pool in self.component_pools.iter_mut() {
            component_pool.create_empty_entity();
        }
    }

    pub fn delete_entity_from_each_component_pool(&mut self, entity: usize) {
        for component_pool in self.component_pools.iter_mut() {
            component_pool.delete_entity(entity);
        }
    }
}
