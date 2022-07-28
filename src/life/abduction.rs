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
                std::any::type_name::<G>().to_string(),
                entity,
            ))? as usize;

        Ok(self.component_array.get(index).unwrap())
    }

    pub fn get_mut(&mut self, entity: usize) -> Result<&mut G, LifeError> {
        let index = *self
            .sparse_array
            .get(entity)
            .ok_or(LifeError::EntityOutOfScope(
                std::any::type_name::<G>().to_string(),
                entity,
            ))? as usize;

        Ok(self.component_array.get_mut(index).unwrap())
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
            Ok(_) => panic!(
                "Scene should not have been able to find f32 component pool with get_trait()"
            ),
        }
    }

    #[test]
    fn requesting_for_non_existing_component_pool_returns_error_mut_version() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        match scene.get_trait_mut::<f32>() {
            Err(e) => assert_eq!(e, LifeError::NoComponentPool(String::from("f32"))),
            Ok(_) => panic!(
                "Scene should not have been able to find f32 component pool with get_trait_mut()"
            ),
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
                "Scene should not have been able to downcast &Box<dyn ComponentPoolTrait> to &ComponentPool<f32> in get()"
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
                "Scene should not have been able to downcast &Box<dyn ComponentPoolTrait> to &ComponentPool<f32> in get_mut()"
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

    #[test]
    fn getting_component_from_entity_thats_out_of_scope_return_error() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        match scene.get_component::<i32>(3) {
            Err(e) => assert_eq!(e, LifeError::EntityOutOfScope(String::from("i32"), 3)),
            Ok(_) => panic!(
                "Scene should not be able to find a component that's out of scope in get_component()"
            ),
        }
    }

    #[test]
    fn getting_component_from_entity_that_isnt_bound_to_any_component_returns_error() {
        let mut scene = Scene::new();

        let type_id = TypeId::of::<i32>();
        let component_pool = Box::new(ComponentPool {
            num_components: 1,

            sparse_array: vec![-1, -1, -1, 1],
            packed_array: vec![3],
            component_array: vec![32],
        });
        scene.component_pools.insert(type_id, component_pool);

        match scene.get_component::<i32>(3) {
            Err(e) => assert_eq!(e, LifeError::EntityNotBoundToComponent(String::from("i32"), 3)),
            Ok(_) => panic!(
                "Scene should not have been able to find component with get_component() since entity 3 is not bound to anything"
            ),
        }
    }

    #[test]
    fn downcast_error_is_correctly_propagated_at_get_component() {
        let mut scene = Scene::new();

        let type_id = TypeId::of::<f32>();
        let component_pool = Box::new(ComponentPool::new_with_entity(1, 32 as i32));
        scene.component_pools.insert(type_id, component_pool);

        match scene.get_component::<f32>(3) {
            Err(e) => assert_eq!(e, LifeError::Downcast(String::from("f32"))),
            Ok(_) => panic!("Error was not propagated successfully from get() to get_component()"),
        }
    }

    #[test]
    fn getting_mut_component_from_entity_thats_out_of_scope_return_error() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        match scene.get_component_mut::<i32>(3) {
            Err(e) => assert_eq!(e, LifeError::EntityOutOfScope(String::from("i32"), 3)),
            Ok(_) => panic!(
                "Scene should not be able to find a component that's out of scope in get_component_mut()"
            ),
        }
    }

    #[test]
    fn getting_mut_component_from_entity_that_isnt_bound_to_any_component_returns_error() {
        let mut scene = Scene::new();

        let type_id = TypeId::of::<i32>();
        let component_pool = Box::new(ComponentPool {
            num_components: 1,

            sparse_array: vec![-1, -1, -1, 1],
            packed_array: vec![3],
            component_array: vec![32],
        });
        scene.component_pools.insert(type_id, component_pool);

        match scene.get_component_mut::<i32>(3) {
            Err(e) => assert_eq!(e, LifeError::EntityNotBoundToComponent(String::from("i32"), 3)),
            Ok(_) => panic!(
                "Scene should not have been able to find component with get_component_mut() since entity 3 is not bound to anything"
            ),
        }
    }

    #[test]
    fn downcast_error_is_correctly_propagated_at_get_component_mut() {
        let mut scene = Scene::new();

        let type_id = TypeId::of::<f32>();
        let component_pool = Box::new(ComponentPool::new_with_entity(1, 32 as i32));
        scene.component_pools.insert(type_id, component_pool);

        match scene.get_component_mut::<f32>(3) {
            Err(e) => assert_eq!(e, LifeError::Downcast(String::from("f32"))),
            Ok(_) => panic!(
                "Error was not propagated successfully from get_mut() to get_component_mut()"
            ),
        }
    }
}
