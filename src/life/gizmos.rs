use std::any::type_name;

use crate::life::{glitch::LifeError, Component, ComponentPool, Scene};

impl<P> Scene<P> {
    pub fn component_exists<G: 'static + Component<P>>(&mut self, entity: usize) -> bool {
        self.get::<G>().has_component(entity)
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
}

pub trait PoolToolbox {
    fn has_component(&self, entity: usize) -> bool;

    fn entity_id(&self, component_id: usize) -> Result<usize, LifeError>;
    fn component_id(&self, entity_id: usize) -> Result<usize, LifeError>;

    fn swap_entities(&mut self, entity: usize, entity_destination: usize);
    fn swap_components(&mut self, component: usize, component_destination: usize);
    fn swap(&mut self, entt: usize, entt_dest: usize, comp: usize, comp_dest: usize);
}

impl<G: 'static> PoolToolbox for ComponentPool<G> {
    fn has_component(&self, entity: usize) -> bool {
        self.sparse_array.get(entity).unwrap_or(&-1) != &-1
    }

    fn entity_id(&self, component_id: usize) -> Result<usize, LifeError> {
        self.packed_array
            .get(component_id)
            .map(|x| x.clone())
            .ok_or(LifeError::ComponentNotFound(type_name::<G>(), component_id))
    }

    fn component_id(&self, entity_id: usize) -> Result<usize, LifeError> {
        match self.sparse_array.get(entity_id) {
            None => Err(LifeError::EntityNotFound(type_name::<G>(), entity_id)),
            Some(&-1) => Err(LifeError::EntityNotBoundToComponent(
                type_name::<G>(),
                entity_id,
            )),
            Some(id) => Ok(id.clone() as usize),
        }
    }

    fn swap_entities(&mut self, entity: usize, entity_destination: usize) {
        let component = unwrap!(self.component_id(entity));
        let component_destination = unwrap!(self.component_id(entity_destination));

        self.swap(entity, entity_destination, component, component_destination);
    }

    fn swap_components(&mut self, component: usize, component_destination: usize) {
        let entity = unwrap!(self.entity_id(component));
        let entity_destination = unwrap!(self.entity_id(component_destination));

        self.swap(entity, entity_destination, component, component_destination);
    }

    fn swap(&mut self, entt: usize, entt_dest: usize, comp: usize, comp_dest: usize) {
        self.sparse_array.swap(entt, entt_dest);
        self.packed_array.swap(comp, comp_dest);
        self.component_array.swap(comp, comp_dest);
    }
}

#[cfg(test)]
mod tests {
    use crate::life::{ComponentPool, PoolToolbox};

    #[test]
    fn entity_out_of_bounds_does_not_have_component() {
        let mut pool = ComponentPool::empty();
        pool.assign_component(3, 32);

        assert_eq!(pool.has_component(4), false);
    }

    #[test]
    fn entity_without_component_does_not_have_component() {
        let mut pool = ComponentPool::empty();
        pool.assign_component(3, 32);

        assert_eq!(pool.has_component(2), false);
    }

    #[test]
    fn entity_with_component_has_component() {
        let mut pool = ComponentPool::empty();
        pool.assign_component(3, 32);

        assert_eq!(pool.has_component(3), true);
    }
}
