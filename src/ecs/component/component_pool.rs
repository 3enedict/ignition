use log::info;

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

        let component_pool = Self {
            num_components: 1,

            sparse_array,
            packed_array,
            component_array,
        };

        component_pool.log_vars_in_component_pool((Vec::new(), Vec::new(), 0));
        component_pool
    }

    pub fn assign_component_to_entity(&mut self, entity: usize, component: G) {
        let snapshot = self.snapshot_for_logs();

        Self::add_entity_to_sparse_array(entity, self.num_components, &mut self.sparse_array);
        self.packed_array.push(entity);
        self.component_array.push(component);
        self.num_components += 1;

        self.log_vars_in_component_pool(snapshot);
    }

    /* Utility functions */

    fn add_entity_to_sparse_array(entity: usize, value: usize, sparse_array: &mut Vec<i32>) {
        if entity + 1 > sparse_array.len() {
            sparse_array.resize(entity + 1, -1);
        }

        sparse_array[entity] = value as i32;
    }

    fn snapshot_for_logs(&self) -> (Vec<i32>, Vec<usize>, usize) {
        (
            self.sparse_array.clone(),
            self.packed_array.clone(),
            self.num_components,
        )
    }

    fn log_vars_in_component_pool(&self, snapshot: (Vec<i32>, Vec<usize>, usize)) {
        let (sparse_array, packed_array, num_components) = snapshot;

        info!(
            "Sparse array: {:?} -> {:?}",
            sparse_array, self.sparse_array
        );
        info!(
            "Packed array: {:?} -> {:?}",
            packed_array, self.packed_array
        );
        info!(
            "Num components: {:?} -> {:?}",
            num_components, self.num_components
        );
    }
}
