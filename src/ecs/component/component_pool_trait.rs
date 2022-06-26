use crate::ecs::component::component_pool::ComponentPool;

pub trait ComponentPoolTrait {
    fn create_empty_entity(&mut self);
    fn delete_entity(&mut self, entity: usize);
    fn disable_entity(&mut self, entity: usize);
    fn enable_entity(&mut self, entity: usize);

    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn move_to_back(&mut self, entity: usize);

    fn swap(&mut self, entity: usize, destination: usize);
    fn swap_components(&mut self, component: usize, destination: usize);
    fn swap_arrays(
        &mut self,
        comp_in_sparse: usize,
        dest_in_sparse: usize,
        comp_in_packed: usize,
        dest_in_packed: usize,
    );
}

impl<G: 'static> ComponentPoolTrait for ComponentPool<G> {
    fn create_empty_entity(&mut self) {
        self.sparse_array.push(-1);
    }

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

    fn disable_entity(&mut self, entity: usize) {
        self.move_to_back(entity);
        self.num_components -= 1;
    }

    fn enable_entity(&mut self, entity: usize) {
        self.num_components += 1;
        self.move_to_back(entity);
    }

    /* Utility functions */

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn move_to_back(&mut self, entity: usize) {
        self.swap(entity, self.sparse_array.len() - 1);
    }

    fn swap(&mut self, entity: usize, destination: usize) {
        let comp_in_packed = self.sparse_array[entity] as usize;
        let dest_in_packed = self.sparse_array[destination] as usize;

        self.swap_arrays(entity, destination, comp_in_packed, dest_in_packed);
    }

    fn swap_components(&mut self, component: usize, destination: usize) {
        let comp_in_sparse = self.packed_array[component];
        let dest_in_sparse = self.packed_array[destination];

        self.swap_arrays(comp_in_sparse, dest_in_sparse, component, destination);
    }

    fn swap_arrays(
        &mut self,
        comp_in_sparse: usize,
        dest_in_sparse: usize,
        comp_in_packed: usize,
        dest_in_packed: usize,
    ) {
        self.sparse_array.swap(comp_in_sparse, dest_in_sparse);
        self.packed_array.swap(comp_in_packed, dest_in_packed);
        self.component_array.swap(comp_in_packed, dest_in_packed);
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::component::{component_pool_trait::ComponentPoolTrait, ComponentPool};

    #[test]
    fn sparse_array_reflects_entity_deletion_correctly() {
        let mut component_pool = ComponentPool::new_with_entity(2, 32);
        component_pool.assign_component_to_entity(4, 64);
        component_pool.assign_component_to_entity(5, 128);

        component_pool.delete_entity(2);

        assert_eq!(
            ComponentPool {
                num_components: 2,

                sparse_array: vec! { -1, -1, -1, -1, 1, 0 },
                packed_array: vec! { 5, 4 },
                component_array: vec! { 128, 64 },
            },
            component_pool,
        );
    }
}
