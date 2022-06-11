#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ComponentPool<G> {
    pub num_components: usize,

    pub sparse_array: Vec<i32>,
    pub packed_array: Vec<usize>,
    pub component_array: Vec<G>,
}

impl<G> ComponentPool<G> {
    pub fn new_with_entity(entity: usize, component: G) -> Self {
        let mut sparse_array = Vec::with_capacity(entity + 1);
        Self::add_entity_to_sparse_array(entity, 0, &mut sparse_array);

        let packed_array = vec![entity];
        let component_array = vec![component];

        Self {
            num_components: 1,

            sparse_array,
            packed_array,
            component_array,
        }
    }

    pub fn assign_component_to_entity(&mut self, entity: usize, component: G) {
        Self::add_entity_to_sparse_array(entity, self.num_components, &mut self.sparse_array);

        self.packed_array.push(entity);
        self.component_array.push(component);
        self.num_components += 1;
    }

    /* Utility functions */

    fn add_entity_to_sparse_array(entity: usize, value: usize, sparse_array: &mut Vec<i32>) {
        if entity + 1 > sparse_array.len() {
            sparse_array.resize(entity + 1, -1);
        }

        sparse_array[entity] = value as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::component::component_pool::ComponentPool;

    #[test]
    fn adding_an_entity_to_sparse_array_fills_the_gaps() {
        let mut sparse_array = vec![-1, -1, 0];
        ComponentPool::<i32>::add_entity_to_sparse_array(5, 1, &mut sparse_array);

        assert_eq!(vec![-1, -1, 0, -1, -1, 1], sparse_array);
    }

    #[test]
    fn recycling_an_entity_in_sparse_array_does_not_resize_it_incorrectly() {
        let mut sparse_array = vec![-1, -1, 0];
        ComponentPool::<i32>::add_entity_to_sparse_array(0, 1, &mut sparse_array);

        assert_eq!(vec![1, -1, 0], sparse_array);
    }
}
