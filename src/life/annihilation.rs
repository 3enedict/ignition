use crate::life::{gizmos::PoolToolbox, glitch::LifeError, ComponentPool, Scene};

impl Scene {
    pub fn delete(&mut self, entity: usize) {
        self.available_entities.push(entity);
        self.delete_entity_from_each_component_pool(entity);
    }

    pub fn delete_entity_from_each_component_pool(&mut self, entity: usize) {
        for component_pool_option in self.component_pools.iter_mut() {
            if let Some(component_pool) = component_pool_option {
                component_pool.delete_entity(entity);
            }
        }
    }
}

impl<G: 'static> ComponentPool<G> {
    pub fn take_entity(&mut self, entity: usize) -> Result<G, LifeError> {
        let component = self.component_id(entity)?;
        let last_index = self.packed_array.last().unwrap().clone();

        self.num_components -= 1;

        self.sparse_array[last_index] = component as i32;
        self.sparse_array[entity] = -1;

        self.packed_array.swap_remove(component);
        Ok(self.component_array.swap_remove(component))
    }
}

pub trait EntityDestructor {
    fn delete_entity(&mut self, entity: usize);
}

impl<G: 'static> EntityDestructor for ComponentPool<G> {
    fn delete_entity(&mut self, entity: usize) {
        unwrap!(self.take_entity(entity));
    }
}

#[cfg(test)]
mod tests {
    use crate::life::{annihilation::EntityDestructor, ComponentPool};

    #[test]
    fn deleting_an_entity_updates_component_pool_correctly() {
        let mut pool = ComponentPool::empty();
        pool.assign_component(1, 32 as i32);
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
    fn deleting_last_entity_doesn_t_swap_with_non_existing_component() {
        let mut pool = ComponentPool::empty();
        pool.assign_component(1, 32 as i32);

        pool.delete_entity(1);

        assert_eq!(
            pool,
            ComponentPool {
                num_components: 0,

                sparse_array: vec![-1, -1],
                packed_array: vec![],
                component_array: vec![],
            },
        );
    }

    #[test]
    fn deleting_a_non_existing_entity_does_nothing() {
        let mut pool = ComponentPool::empty();
        pool.assign_component(1, 32 as i32);
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

    #[test]
    fn deleting_two_entities_works_as_expected() {
        let mut pool = ComponentPool::empty();
        pool.assign_component(1, 32 as i32);
        pool.assign_component(3, 81 as i32);

        pool.delete_entity(1);
        pool.delete_entity(3);

        assert_eq!(
            pool,
            ComponentPool {
                num_components: 0,

                sparse_array: vec![-1, -1, -1, -1],
                packed_array: vec![],
                component_array: vec![],
            },
        );
    }
}
