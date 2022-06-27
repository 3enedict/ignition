use crate::ecs::component::component_pool::ComponentPool;

pub trait ComponentPoolTrait {
    fn create_empty_entity(&mut self);
    fn delete_entity(&mut self, entity: usize);
    fn disable_entity(&mut self, entity: usize);
    fn enable_entity(&mut self, entity: usize);

    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn move_to_back(&mut self, entity: usize);

    fn swap_entities(&mut self, entity: usize, destination: usize);
    fn swap_components(&mut self, component: usize, destination: usize);
    fn swap(
        &mut self,
        entity: usize,
        entity_destination: usize,
        component: usize,
        component_destination: usize,
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
        let component = self.sparse_array[entity] as usize;
        let component_destination = self.num_components - 1;

        let entity_destination = self.packed_array[component_destination];

        self.swap(entity, entity_destination, component, component_destination);
    }

    fn swap_entities(&mut self, entity: usize, entity_destination: usize) {
        let component = self.sparse_array[entity] as usize;
        let component_destination = self.sparse_array[entity_destination] as usize;

        self.swap(entity, entity_destination, component, component_destination);
    }

    fn swap_components(&mut self, component: usize, component_destination: usize) {
        let entity = self.packed_array[component];
        let entity_destination = self.packed_array[component_destination];

        self.swap(entity, entity_destination, component, component_destination);
    }

    fn swap(
        &mut self,
        entity: usize,
        entity_destination: usize,
        component: usize,
        component_destination: usize,
    ) {
        self.sparse_array.swap(entity, entity_destination);
        self.packed_array.swap(component, component_destination);
        self.component_array.swap(component, component_destination);
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::component::{component_pool_trait::ComponentPoolTrait, ComponentPool};

    #[test]
    fn sparse_array_reflects_entity_deletion_correctly() {
        let mut component_pool = ComponentPool::new_with_entity(2, 32);
        component_pool.assign_component(4, 64);
        component_pool.assign_component(5, 128);

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
