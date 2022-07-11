use std::collections::HashMap;

pub mod component;
pub mod entity;

use crate::life::{ComponentPool, Scene};

impl Scene {
    pub fn new() -> Self {
        Self {
            available_entities: vec![0],
            component_pools: HashMap::new(),
        }
    }
}

impl<G> ComponentPool<G> {
    pub fn new_with_entity(entity: usize, component: G) -> Self {
        let mut sparse_array = Vec::with_capacity(entity + 1);
        Self::add_entity_to_sparse_array(entity, 0, &mut sparse_array);

        Self {
            num_components: 1,

            sparse_array,
            packed_array: vec![entity],
            component_array: vec![component],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::life::ComponentPool;

    #[test]
    fn component_pool_creation_works() {
        let pool = ComponentPool::new_with_entity(3, 32);

        assert_eq!(
            pool,
            ComponentPool {
                num_components: 1,

                sparse_array: vec![-1, -1, -1, 0],
                packed_array: vec![3],
                component_array: vec![32],
            },
        );
    }
}
