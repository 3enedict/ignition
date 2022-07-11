use crate::life::{ComponentPool, Scene};

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

#[cfg(test)]
mod tests {
    use crate::life::{annihilation::EntityDestructor, ComponentPool};

    #[test]
    fn deleting_an_entity_updates_component_pool_correctly() {
        let mut pool = ComponentPool::new_with_entity(1, 32 as i32);
        pool.assign_component(2, 21 as i32);

        pool.delete_entity(1);

        assert_eq!(
            pool,
            ComponentPool {
                num_components: 1,

                sparse_array: vec![-1, -1, 0],
                packed_array: vec![2],
                component_array: vec![21],
            },
        );
    }

    #[test]
    fn deleting_a_non_existing_entity_does_nothing() {
        let mut pool = ComponentPool::new_with_entity(1, 32 as i32);
        pool.delete_entity(0);

        assert_eq!(
            pool,
            ComponentPool {
                num_components: 1,

                sparse_array: vec![-1, 0],
                packed_array: vec![1],
                component_array: vec![32],
            },
        );
    }
}
