use crate::life::{ComponentPool, ComponentPoolTrait, Scene};
use std::any::TypeId;

use super::glitch::LifeError;

impl Scene {
    pub fn get_trait<G: 'static>(&self) -> Result<&Box<dyn ComponentPoolTrait>, LifeError> {
        self.component_pools
            .get(&TypeId::of::<G>())
            .ok_or(LifeError::NoComponentPool(
                std::any::type_name::<G>().to_string(),
            ))
    }

    pub fn get_trait_mut<G: 'static>(
        &mut self,
    ) -> Result<&mut Box<dyn ComponentPoolTrait>, LifeError> {
        self.component_pools
            .get_mut(&TypeId::of::<G>())
            .ok_or(LifeError::NoComponentPool(
                std::any::type_name::<G>().to_string(),
            ))
    }

    pub fn get<G: 'static>(&self) -> Result<&ComponentPool<G>, LifeError> {
        self.get_trait::<G>()?
            .as_any()
            .downcast_ref::<ComponentPool<G>>()
            .ok_or(LifeError::Downcast(std::any::type_name::<G>().to_string()))
    }

    pub fn get_mut<G: 'static>(&mut self) -> Result<&mut ComponentPool<G>, LifeError> {
        self.get_trait_mut::<G>()?
            .as_any_mut()
            .downcast_mut::<ComponentPool<G>>()
            .ok_or(LifeError::Downcast(std::any::type_name::<G>().to_string()))
    }

    pub fn get_component<G: 'static>(&self, entity: usize) -> &G {
        self.get::<G>().unwrap().get(entity)
    }

    pub fn get_component_mut<G: 'static>(&mut self, entity: usize) -> &mut G {
        self.get_mut::<G>().unwrap().get_mut(entity)
    }

    pub fn take_component<G: 'static>(&mut self, entity: usize) -> G {
        self.get_mut::<G>().unwrap().take_entity(entity).unwrap()
    }

    pub fn get_current_entity(&self) -> usize {
        self.available_entities[self.available_entities.len() - 1]
    }
}

impl<G> ComponentPool<G> {
    pub fn get(&self, entity: usize) -> &G {
        let index = self.sparse_array[entity] as usize;
        self.component_array.get(index).unwrap()
    }

    pub fn get_mut(&mut self, entity: usize) -> &mut G {
        let index = self.sparse_array[entity] as usize;
        self.component_array.get_mut(index).unwrap()
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
    use std::any::TypeId;

    use crate::life::{glitch::LifeError, ComponentPool, Scene};

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

    #[test]
    fn requesting_for_non_existing_component_pool_returns_error() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        match scene.get_trait::<f32>() {
            Err(e) => assert_eq!(e, LifeError::NoComponentPool(String::from("f32"))),
            Ok(_) => panic!("Test should not have found f32 in scene"),
        }
    }

    #[test]
    fn requesting_for_non_existing_component_pool_returns_error_mut_version() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        match scene.get_trait_mut::<f32>() {
            Err(e) => assert_eq!(e, LifeError::NoComponentPool(String::from("f32"))),
            Ok(_) => panic!("Test should not have found f32 in scene"),
        }
    }

    #[test]
    fn unable_to_downcast_component_pool_trait_returns_error() {
        let mut scene = Scene::new();

        let type_id = TypeId::of::<f32>();
        let component_pool = Box::new(ComponentPool::new_with_entity(1, 32 as i32));
        scene.component_pools.insert(type_id, component_pool);

        match scene.get::<f32>() {
            Err(e) => assert_eq!(e, LifeError::Downcast(String::from("f32"))),
            Ok(_) => panic!(
                "Scene should not have been able to downcast &Box<dyn ComponentPoolTrait> for f32"
            ),
        }
    }

    #[test]
    fn no_component_pool_error_is_correctly_propagated_at_get() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        match scene.get::<f32>() {
            Err(e) => assert_eq!(e, LifeError::NoComponentPool(String::from("f32"))),
            Ok(_) => panic!("Error was not propagated successfully from get_trait() to get()"),
        }
    }

    #[test]
    fn unable_to_downcast_component_pool_trait_returns_error_mut_version() {
        let mut scene = Scene::new();

        let type_id = TypeId::of::<f32>();
        let component_pool = Box::new(ComponentPool::new_with_entity(1, 32 as i32));
        scene.component_pools.insert(type_id, component_pool);

        match scene.get_mut::<f32>() {
            Err(e) => assert_eq!(e, LifeError::Downcast(String::from("f32"))),
            Ok(_) => panic!(
                "Scene should not have been able to downcast &mut Box<dyn ComponentPoolTrait> for f32"
            ),
        }
    }

    #[test]
    fn no_component_pool_error_is_correctly_propagated_at_get_mut() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        match scene.get_mut::<f32>() {
            Err(e) => assert_eq!(e, LifeError::NoComponentPool(String::from("f32"))),
            Ok(_) => {
                panic!("Error was not propagated successfully from get_trait_mut() to get_mut()")
            }
        }
    }
}
