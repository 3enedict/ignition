use crate::life::{ComponentPool, Scene};
use std::any::TypeId;

impl Scene {
    pub fn component_pool_exists<G: 'static>(&mut self) -> bool {
        self.component_pools.contains_key(&TypeId::of::<G>())
    }

    pub fn component_exists<G: 'static>(&mut self, entity: usize) -> bool {
        self.component_pool_exists::<G>() && self.get::<G>().has_component(entity)
    }
}

impl<G> ComponentPool<G> {
    pub fn iter(&self) -> impl Iterator<Item = &G> {
        let (left, _right) = self.component_array.split_at(self.num_components);

        left.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut G> {
        let (left, _right) = self.component_array.split_at_mut(self.num_components);

        left.iter_mut()
    }

    pub fn has_component(&self, entity: usize) -> bool {
        self.sparse_array.get(entity).unwrap_or(&-1) != &-1
    }
}

pub trait PoolToolbox {
    fn swap_entities(&mut self, entity: usize, entity_destination: usize);
    fn swap_components(&mut self, component: usize, component_destination: usize);
    fn swap(
        &mut self,
        entity: usize,
        entity_destination: usize,
        component: usize,
        component_destination: usize,
    );
}

impl<G: 'static> PoolToolbox for ComponentPool<G> {
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
    use crate::life::ComponentPool;

    #[test]
    fn entity_out_of_bounds_does_not_have_component() {
        let pool = ComponentPool::new_with_entity(3, 32);

        assert_eq!(pool.has_component(4), false);
    }

    #[test]
    fn entity_without_component_does_not_have_component() {
        let pool = ComponentPool::new_with_entity(3, 32);

        assert_eq!(pool.has_component(2), false);
    }

    #[test]
    fn entity_with_component_has_component() {
        let pool = ComponentPool::new_with_entity(3, 32);

        assert_eq!(pool.has_component(3), true);
    }
}
