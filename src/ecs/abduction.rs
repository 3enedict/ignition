use crate::ecs::{ComponentPool, ComponentPoolTrait, Scene};
use std::any::TypeId;

impl Scene {
    pub fn get_trait<G: 'static>(&self) -> &Box<dyn ComponentPoolTrait> {
        self.component_pools.get(&TypeId::of::<G>()).unwrap()
    }

    pub fn get_trait_mut<G: 'static>(&mut self) -> &mut Box<dyn ComponentPoolTrait> {
        self.component_pools.get_mut(&TypeId::of::<G>()).unwrap()
    }

    pub fn get<G: 'static>(&self) -> &ComponentPool<G> {
        self.get_trait::<G>()
            .as_any()
            .downcast_ref::<ComponentPool<G>>()
            .unwrap()
    }

    pub fn get_mut<G: 'static>(&mut self) -> &mut ComponentPool<G> {
        self.get_trait_mut::<G>()
            .as_any_mut()
            .downcast_mut::<ComponentPool<G>>()
            .unwrap()
    }

    pub fn get_component<G: 'static>(&self, entity: usize) -> &G {
        self.get::<G>().component_array.get(entity).unwrap()
    }

    pub fn get_component_mut<G: 'static>(&mut self, entity: usize) -> &mut G {
        self.get_mut::<G>().component_array.get_mut(entity).unwrap()
    }

    pub fn get_current_entity(&self) -> usize {
        self.available_entities[self.available_entities.len() - 1]
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
    use crate::prelude::Scene;

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
