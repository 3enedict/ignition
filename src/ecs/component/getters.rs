use crate::ecs::{component::ComponentPool, Scene};
use std::any::TypeId;

use super::component_pool_trait::ComponentPoolTrait;

impl Scene {
    pub fn get_trait<G: 'static>(&self) -> &Box<dyn ComponentPoolTrait> {
        self.component_pools
            .get(*self.component_indices.get(&TypeId::of::<G>()).unwrap())
            .unwrap()
    }

    pub fn get_trait_mut<G: 'static>(&mut self) -> &mut Box<dyn ComponentPoolTrait> {
        self.component_pools
            .get_mut(*self.component_indices.get(&TypeId::of::<G>()).unwrap())
            .unwrap()
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

    pub fn get_component_mut<G: 'static>(&mut self, entity: usize) -> &G {
        self.get_mut::<G>().component_array.get(entity).unwrap()
    }
}
