use crate::life::{ComponentPool, ComponentPoolTrait, Scene};
use std::any::TypeId;

use super::glitch::LifeError;

impl Scene {
    pub fn get_trait<G: 'static>(&self) -> Result<&Box<dyn ComponentPoolTrait>, LifeError> {
        self.component_pools
            .get(&TypeId::of::<G>())
            .ok_or(LifeError::NoComponentPool(std::any::type_name::<G>()))
    }

    pub fn get_trait_mut<G: 'static>(
        &mut self,
    ) -> Result<&mut Box<dyn ComponentPoolTrait>, LifeError> {
        self.component_pools
            .get_mut(&TypeId::of::<G>())
            .ok_or(LifeError::NoComponentPool(std::any::type_name::<G>()))
    }

    pub fn get<G: 'static>(&self) -> Result<&ComponentPool<G>, LifeError> {
        self.get_trait::<G>()?
            .as_any()
            .downcast_ref::<ComponentPool<G>>()
            .ok_or(LifeError::Downcast(std::any::type_name::<G>()))
    }

    pub fn get_mut<G: 'static>(&mut self) -> Result<&mut ComponentPool<G>, LifeError> {
        self.get_trait_mut::<G>()?
            .as_any_mut()
            .downcast_mut::<ComponentPool<G>>()
            .ok_or(LifeError::Downcast(std::any::type_name::<G>()))
    }

    pub fn get_component<G: 'static>(&self, entity: usize) -> Result<&G, LifeError> {
        self.get::<G>()?.get(entity)
    }

    pub fn get_component_mut<G: 'static>(&mut self, entity: usize) -> Result<&mut G, LifeError> {
        self.get_mut::<G>()?.get_mut(entity)
    }

    pub fn take_component<G: 'static>(&mut self, entity: usize) -> G {
        self.get_mut::<G>().unwrap().take_entity(entity).unwrap()
    }

    pub fn get_current_entity(&self) -> usize {
        self.available_entities[self.available_entities.len() - 1]
    }
}

impl<G> ComponentPool<G> {
    pub fn get(&self, entity: usize) -> Result<&G, LifeError> {
        let index = *self
            .sparse_array
            .get(entity)
            .ok_or(LifeError::EntityOutOfScope(
                std::any::type_name::<G>(),
                entity,
            ))? as usize;

        self.component_array
            .get(index)
            .ok_or(LifeError::EntityNotBoundToComponent(
                std::any::type_name::<G>(),
                entity,
            ))
    }

    pub fn get_mut(&mut self, entity: usize) -> Result<&mut G, LifeError> {
        let index = *self
            .sparse_array
            .get(entity)
            .ok_or(LifeError::EntityOutOfScope(
                std::any::type_name::<G>(),
                entity,
            ))? as usize;

        self.component_array
            .get_mut(index)
            .ok_or(LifeError::EntityNotBoundToComponent(
                std::any::type_name::<G>(),
                entity,
            ))
    }
}

impl<G: 'static> ComponentPoolTrait for ComponentPool<G> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

#[cfg(test)]
mod tests {
    use crate::life::Scene;

    #[test]
    fn calling_get_current_entity_returns_correct_id() {
        let scene = Scene::new();

        assert_eq!(0, scene.get_current_entity());
    }

    #[test]
    fn calling_get_current_entity_returns_correct_id_even_with_recycled_entities() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.entity();

        scene.delete(entity);

        assert_eq!(0, scene.get_current_entity());
    }
}
