use crate::ecs::{ComponentPool, Scene};

impl Scene {
    pub fn delete(&mut self, entity: usize) {
        self.available_entities.push(entity);
        self.delete_entity_from_each_component_pool(entity);
    }

    pub fn delete_entity_from_each_component_pool(&mut self, entity: usize) {
        for (_type_id, component_pool) in self.component_pools.iter_mut() {
            component_pool.delete_entity(entity);
        }
    }
}

pub trait EntityDestructor {
    fn delete_entity(&mut self, entity: usize);
}

impl<G: 'static> EntityDestructor for ComponentPool<G> {
    fn delete_entity(&mut self, entity: usize) {
        let index = self.sparse_array[entity];

        if index != -1 {
            self.num_components -= 1;

            self.packed_array.swap_remove(index as usize);
            self.component_array.swap_remove(index as usize);

            let last_index = self.sparse_array.len() - 1;
            self.sparse_array[last_index] = self.sparse_array[entity];
            self.sparse_array[entity] = -1;

            self.packed_array[index as usize] = last_index;
        }
    }
}
