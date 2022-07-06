use std::any::TypeId;
use std::collections::HashMap;

use crate::ecs::{ComponentPool, Scene};

impl Scene {
    pub fn new() -> Self {
        Self {
            available_entities: vec![0],
            component_pools: HashMap::new(),
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
        let type_id = TypeId::of::<G>();
        let component_pool = Box::new(ComponentPool::new_with_entity(entity, component));

        self.component_pools.insert(type_id, component_pool);
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
    use crate::ecs::Scene;

    #[test]
    fn creating_entities_simply_increments_an_id() {
        let mut scene = Scene::new();
        let mut entities: Vec<usize> = Vec::new();

        for _i in 0..10 {
            entities.push(scene.entity());
        }

        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], entities);
    }

    #[test]
    fn creating_an_entity_after_having_deleted_one_uses_recycled_id() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.delete(entity);

        assert_eq!(0, scene.entity());
    }

    #[test]
    fn creating_new_component_pool_updates_scene() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 34 as i32);

        assert_eq!(scene.get::<i32>().iter().collect::<Vec<&i32>>(), vec![&34]);
    }

    #[test]
    fn assigning_component_updates_scene() {
        let mut scene = Scene::new();

        let entity1 = scene.entity();
        scene.component(entity1, 34 as i32);

        let entity2 = scene.entity();
        scene.component(entity2, 25 as i32);

        assert_eq!(
            scene.get::<i32>().iter().collect::<Vec<&i32>>(),
            vec![&34, &25]
        );
    }
}
