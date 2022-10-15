use std::any::type_name;

use crate::life::{
    gizmos::PoolToolbox, glitch::LifeError, Component, ComponentPool, ComponentPoolTrait, Scene,
};

impl Scene {
    pub fn get_trait<G: 'static + Component>(
        &self,
    ) -> Result<&Box<dyn ComponentPoolTrait>, LifeError> {
        self.component_pools
            .get(G::id())
            .ok_or(LifeError::NoComponentPool(type_name::<G>()))?
            .as_ref()
            .ok_or(LifeError::NoComponentPool(type_name::<G>()))
    }

    pub fn get_trait_mut<G: 'static + Component>(
        &mut self,
    ) -> Result<&mut Box<dyn ComponentPoolTrait>, LifeError> {
        self.component_pools
            .get_mut(G::id())
            .ok_or(LifeError::NoComponentPool(type_name::<G>()))?
            .as_mut()
            .ok_or(LifeError::NoComponentPool(type_name::<G>()))
    }

    pub fn get<G: 'static + Component>(&self) -> Result<&ComponentPool<G>, LifeError> {
        self.get_trait::<G>()?
            .as_any()
            .downcast_ref::<ComponentPool<G>>()
            .ok_or(LifeError::Downcast(type_name::<G>()))
    }

    pub fn get_mut<G: 'static + Component>(&mut self) -> Result<&mut ComponentPool<G>, LifeError> {
        self.get_trait_mut::<G>()?
            .as_any_mut()
            .downcast_mut::<ComponentPool<G>>()
            .ok_or(LifeError::Downcast(type_name::<G>()))
    }

    pub fn get_component<G: 'static + Component>(&self, entity: usize) -> Result<&G, LifeError> {
        self.get::<G>()?.get(entity)
    }

    pub fn get_component_mut<G: 'static + Component>(
        &mut self,
        entity: usize,
    ) -> Result<&mut G, LifeError> {
        self.get_mut::<G>()?.get_mut(entity)
    }

    pub fn take_component<G: 'static + Component>(
        &mut self,
        entity: usize,
    ) -> Result<G, LifeError> {
        self.get_mut::<G>()?.take_entity(entity)
    }

    pub fn get_current_entity(&self) -> usize {
        self.available_entities[self.available_entities.len() - 1]
    }
}

impl<G: 'static> ComponentPool<G> {
    pub fn get(&self, entity: usize) -> Result<&G, LifeError> {
        let id = self.component_id(entity)?;

        self.component_array
            .get(id)
            .ok_or(LifeError::EntityBoundToNonExistingComponent(
                type_name::<G>(),
                entity,
            ))
    }

    pub fn get_mut(&mut self, entity: usize) -> Result<&mut G, LifeError> {
        let id = self.component_id(entity)?;

        self.component_array
            .get_mut(id)
            .ok_or(LifeError::EntityBoundToNonExistingComponent(
                type_name::<G>(),
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
