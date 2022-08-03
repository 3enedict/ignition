use std::any::TypeId;

use crate::life::{ComponentPool, PoolToolbox, Scene};

impl Scene {
    pub fn component<G: 'static>(&mut self, entity: usize, component: G) {
        if self.component_pool_exists::<G>() {
            self.assign_component(entity, component);
        } else {
            self.new_component_pool(entity, component);
        }
    }

    pub fn vectorized_component<G: 'static>(&mut self, entity: usize, component: G) {
        if self.component_exists::<Vec<G>>(entity) {
            unwrap!(self.get_component_mut::<Vec<G>>(entity)).push(component);
        } else {
            self.component(entity, vec![component]);
        }
    }

    pub fn assign_component<G: 'static>(&mut self, entity: usize, component: G) {
        unwrap!(self.get_mut::<G>()).assign_component(entity, component);
    }

    pub fn new_component_pool<G: 'static>(&mut self, entity: usize, component: G) {
        let type_id = TypeId::of::<G>();
        let component_pool = Box::new(ComponentPool::new_with_entity(entity, component));

        self.component_pools.insert(type_id, component_pool);
    }
}

impl<G: 'static> ComponentPool<G> {
    pub fn assign_component(&mut self, entity: usize, component: G) {
        if self.has_component(entity) {
            *unwrap!(self.get_mut(entity)) = component;
        } else {
            Self::add_entity_to_sparse_array(entity, self.num_components, &mut self.sparse_array);

            self.packed_array.push(entity);
            self.component_array.push(component);
            self.num_components += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use log::Level;
    extern crate testing_logger;

    use crate::life::Scene;

    #[test]
    fn creating_new_component_pool_updates_scene() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 34 as i32);

        assert_eq!(
            scene.get::<i32>().unwrap().iter().collect::<Vec<&i32>>(),
            vec![&34]
        );
    }

    #[test]
    fn assigning_component_in_previously_created_component_pool_updates_scene() {
        let mut scene = Scene::new();

        let entity1 = scene.entity();
        scene.component(entity1, 34 as i32);

        let entity2 = scene.entity();
        scene.component(entity2, 25 as i32);

        assert_eq!(
            scene.get::<i32>().unwrap().iter().collect::<Vec<&i32>>(),
            vec![&34, &25]
        );
    }

    #[test]
    fn assigning_already_existing_component_modifies_current_component() {
        let mut scene = Scene::new();
        let entity = scene.entity();

        scene.component(entity, 34 as i32);
        scene.component(entity, 25 as i32);

        assert_eq!(
            scene.get::<i32>().unwrap().iter().collect::<Vec<&i32>>(),
            vec![&25]
        );
    }

    #[test]
    fn creating_vectorized_component_encapsulates_it_in_vector() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.vectorized_component(entity, 34 as i32);

        assert_eq!(scene.component_pool_exists::<Vec<i32>>(), true);
    }

    #[test]
    fn adding_to_vectorized_component_pushes_to_vector() {
        let mut scene = Scene::new();

        let entity1 = scene.entity();
        scene.vectorized_component(entity1, 34 as i32);
        scene.vectorized_component(entity1, 81 as i32);

        let entity2 = scene.entity();
        scene.vectorized_component(entity2, 63 as u32);
        scene.vectorized_component(entity2, 16 as u32);

        assert_eq!(
            scene
                .get::<Vec<i32>>()
                .unwrap()
                .iter()
                .collect::<Vec<&Vec<i32>>>(),
            vec![&vec![34, 81]]
        );

        assert_eq!(
            scene
                .get::<Vec<u32>>()
                .unwrap()
                .iter()
                .collect::<Vec<&Vec<u32>>>(),
            vec![&vec![63, 16]]
        );
    }

    #[test]
    fn error_in_vectorized_component_is_logged() {
        testing_logger::setup();

        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, vec![2 as i32]);

        scene.get_mut::<Vec<i32>>().unwrap().sparse_array[0] = 1;

        scene.vectorized_component(entity, 3 as i32);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, Level::Warn);

            assert!(captured_logs[0]
                .body
                .contains("Entity 0 is bound to a non existing component for"));
            assert!(captured_logs[0].body.contains("i32")); // The reason why i'm pattern matching against i32 instead of using type_name::<i32>() is because the latter isn't reliable as said in the docs
        });
    }

    #[test]
    fn error_in_assign_component_is_logged() {
        testing_logger::setup();

        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.assign_component(entity, 3 as i32);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, Level::Warn);

            assert!(captured_logs[0]
                .body
                .contains("There's no component pool for"));
            assert!(captured_logs[0].body.contains("i32")); // The reason why i'm pattern matching against i32 instead of using type_name::<i32>() is because the latter isn't reliable as said in the docs
        });
    }
}
