#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ComponentPool<G> {
    pub num_components: i32,

    pub sparse_array: Vec<i32>,
    pub packed_array: Vec<usize>,
    pub component_array: Vec<G>,
}

impl<G> ComponentPool<G> {
    pub fn new_with_entity(entity: usize, component: G) -> Self {
        let mut sparse_array = Self::create_sparse_array_filled_with_empty_entities(entity);
        sparse_array.push(0);

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
        self.sparse_array[entity] = self.num_components;
        self.packed_array.push(entity);
        self.component_array.push(component);

        self.num_components += 1;
    }

    /* Utility functions */

    fn create_sparse_array_filled_with_empty_entities(size: usize) -> Vec<i32> {
        let mut sparse_array = Vec::with_capacity(size);
        sparse_array.resize_with(size, || -1);

        sparse_array
    }
}
