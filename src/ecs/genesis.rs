use std::any::TypeId;
use std::collections::HashMap;

use crate::ecs::{ComponentPool, Scene};

impl Scene {
    pub fn new() -> Self {
        Self {
            available_entities: vec![0],

            component_indices: HashMap::new(),
            component_pools: Vec::new(),
        }
    }

    /* Entity */

    pub fn entity(&mut self) -> usize {
        if self.available_entities.len() == 1 {
            self.generate_new_entity()
        } else {
            self.use_recycled_entity()
        }
    }

    pub fn generate_new_entity(&mut self) -> usize {
        let id = self.available_entities[0];
        self.available_entities[0] += 1;

        id
    }

    pub fn use_recycled_entity(&mut self) -> usize {
        self.available_entities.pop().unwrap()
    }

    /* Component */

    pub fn component<G: 'static>(&mut self, entity: usize, component: G) {
        if self.component_exists::<G>() {
            self.assign_component(entity, component)
        } else {
            self.new_component_pool(entity, component);
        }
    }

    pub fn assign_component<G: 'static>(&mut self, entity: usize, component: G) {
        self.get_mut::<G>().assign_component(entity, component);
    }

    pub fn new_component_pool<G: 'static>(&mut self, entity: usize, component: G) {
        self.push_new_component_index::<G>();
        self.push_new_component_pool(entity, component);
    }

    pub fn push_new_component_index<G: 'static>(&mut self) {
        let type_id = TypeId::of::<G>();
        let index = self.component_pools.len();
        self.component_indices.insert(type_id, index);
    }

    pub fn push_new_component_pool<G: 'static>(&mut self, entity: usize, component: G) {
        let component_pool = Box::new(ComponentPool::new_with_entity(entity, component));
        self.component_pools.push(component_pool);
    }
}

impl<G> ComponentPool<G> {
    pub fn new_with_entity(entity: usize, component: G) -> Self {
        let mut sparse_array = Vec::with_capacity(entity + 1);
        Self::add_entity_to_sparse_array(entity, 0, &mut sparse_array);

        Self {
            num_components: 1,

            sparse_array,
            packed_array: vec![entity],
            component_array: vec![component],
        }
    }

    pub fn assign_component(&mut self, entity: usize, component: G) {
        Self::add_entity_to_sparse_array(entity, self.num_components, &mut self.sparse_array);

        self.packed_array.push(entity);
        self.component_array.push(component);
        self.num_components += 1;
    }

    pub fn add_entity_to_sparse_array(entity: usize, value: usize, sparse_array: &mut Vec<i32>) {
        Self::prolong_sparse_array(entity, sparse_array);
        sparse_array[entity] = value as i32;
    }

    pub fn prolong_sparse_array(entity: usize, sparse_array: &mut Vec<i32>) {
        if entity + 1 > sparse_array.len() {
            sparse_array.resize(entity + 1, -1);
        }
    }
}

pub trait EntityConstructor {
    fn create_empty_entity(&mut self);
}

impl<G: 'static> EntityConstructor for ComponentPool<G> {
    fn create_empty_entity(&mut self) {
        self.sparse_array.push(-1);
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::{ComponentPool, Scene};

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Pos {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Vel {
        speed: u32,
    }

    fn init_four_entities() -> (Scene, usize, usize, usize, usize) {
        let mut scene = Scene::new();

        // Use older format in the name of backwards compatibility...
        let entity1 = scene.entity();
        scene.component(entity1, Vel { speed: 286 });

        let entity2 = scene.entity();

        let entity3 = scene.entity();
        scene.component(entity3, Pos { x: 1, y: -3 });
        scene.component(entity3, Vel { speed: 30 });

        let entity4 = scene.entity();

        (scene, entity1, entity2, entity3, entity4)
    }

    #[test]
    fn creating_components_results_in_correct_storage_inside_the_component_pool() {
        let (scene, _entity1, _entity2, _entity3, _entity4) = init_four_entities();

        assert_eq!(
            &mut ComponentPool {
                num_components: 1,

                sparse_array: vec! { -1, -1, 0 },
                packed_array: vec! { 2 },
                component_array: vec! { Pos { x: 1, y: -3 } },
            },
            scene.get::<Pos>()
        );

        assert_eq!(
            &mut ComponentPool {
                num_components: 2,

                sparse_array: vec! { 0, -1, 1 },
                packed_array: vec! { 0, 2 },
                component_array: vec! { Vel { speed: 286 }, Vel { speed: 30 } },
            },
            scene.get::<Vel>()
        );
    }

    #[test]
    fn deleting_a_component_is_reflected_in_its_component_pool() {
        let (mut scene, entity1, _entity2, _entity3, _entity4) = init_four_entities();

        scene.delete(entity1);

        let entity4 = scene.entity();
        scene.component(entity4, Pos { x: 26, y: 39 });

        assert_eq!(
            &mut ComponentPool {
                num_components: 2,

                sparse_array: vec! { 1, -1, 0 },
                packed_array: vec! { 2, 0 },
                component_array: vec! { Pos { x: 1, y: -3 }, Pos { x: 26, y: 39 } },
            },
            scene.get::<Pos>()
        );

        assert_eq!(
            &mut ComponentPool {
                num_components: 1,

                sparse_array: vec! { -1, -1, 0 },
                packed_array: vec! { 2 },
                component_array: vec! { Vel { speed: 30 } },
            },
            scene.get::<Vel>()
        );
    }

    #[test]
    fn generating_new_entity_id_returns_correct_id() {
        let mut scene = Scene::new();

        assert_eq!(0, scene.generate_new_entity());
    }

    #[test]
    fn generating_new_entity_id_changes_the_list_of_available_entities_correctly() {
        let mut scene = Scene::new();
        scene.generate_new_entity();

        assert_eq!(1, scene.available_entities[0]);
    }

    #[test]
    fn generating_an_entity_uses_recycled_ids_if_available() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.delete(entity);
        let recycled_entity = scene.entity();

        assert_eq!(0, recycled_entity);
    }

    #[test]
    fn adding_an_entity_to_sparse_array_fills_the_gaps() {
        let mut sparse_array = vec![-1, -1, 0];
        ComponentPool::<i32>::add_entity_to_sparse_array(5, 1, &mut sparse_array);

        assert_eq!(vec![-1, -1, 0, -1, -1, 1], sparse_array);
    }

    #[test]
    fn adding_an_entity_inside_sparse_array_does_not_overwrite_parts_of_it() {
        let mut sparse_array = vec![-1, -1, 0];
        ComponentPool::<i32>::add_entity_to_sparse_array(0, 1, &mut sparse_array);

        assert_eq!(vec![1, -1, 0], sparse_array);
    }
}
