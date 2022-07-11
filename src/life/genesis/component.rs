use std::any::TypeId;

use crate::life::{ComponentPool, Scene};

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
            self.get_component_mut::<Vec<G>>(entity).push(component);
        } else {
            self.component(entity, vec![component]);
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
    pub fn assign_component(&mut self, entity: usize, component: G) {
        if self.has_component(entity) {
            self.component_array[self.sparse_array[entity] as usize] = component;
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
    use crate::life::{genesis::entity::EntityConstructor, ComponentPool, Scene};

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

    #[test]
    fn assigning_already_existing_component_modifies_current_component() {
        let mut scene = Scene::new();
        let entity = scene.entity();

        scene.component(entity, 34 as i32);
        scene.component(entity, 25 as i32);

        assert_eq!(scene.get::<i32>().iter().collect::<Vec<&i32>>(), vec![&25]);
    }

    #[test]
    fn assigning_component_updates_component_pool() {
        let mut pool = ComponentPool::new_with_entity(3, 32);
        pool.assign_component(6, 28);

        assert_eq!(
            pool,
            ComponentPool {
                num_components: 2,

                sparse_array: vec![-1, -1, -1, 0, -1, -1, 1],
                packed_array: vec![3, 6],
                component_array: vec![32, 28],
            },
        );
    }

    #[test]
    fn assigning_already_existing_component_does_not_add_component() {
        let mut pool = ComponentPool::new_with_entity(0, 32);
        pool.assign_component(0, 28);

        assert_eq!(
            pool,
            ComponentPool {
                num_components: 1,

                sparse_array: vec![0],
                packed_array: vec![0],
                component_array: vec![28],
            },
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

        let entity = scene.entity();
        scene.vectorized_component(entity, 34 as i32);
        scene.vectorized_component(entity, 59 as i32);

        assert_eq!(
            scene.get::<Vec<i32>>().iter().collect::<Vec<&Vec<i32>>>(),
            vec![&vec![34, 59]]
        );
    }

    #[test]
    fn adding_to_second_vectorized_component_pushes_to_vector() {
        let mut scene = Scene::new();

        let entity1 = scene.entity();
        scene.vectorized_component(entity1, 34 as i32);
        scene.vectorized_component(entity1, 59 as i32);

        let entity2 = scene.entity();
        scene.vectorized_component(entity2, 63 as i32);
        scene.vectorized_component(entity2, 16 as i32);

        assert_eq!(
            scene.get::<Vec<i32>>().iter().collect::<Vec<&Vec<i32>>>(),
            vec![&vec![34, 59], &vec![63, 16]]
        );
    }

    #[test]
    fn adding_differently_typed_vectorized_components_does_not_crash() {
        let mut scene = Scene::new();

        let entity1 = scene.entity();
        scene.vectorized_component(entity1, 34 as i32);
        scene.vectorized_component(entity1, 0.59 as f32);
        scene.vectorized_component(entity1, 81 as i32);

        let entity2 = scene.entity();
        scene.vectorized_component(entity2, 63 as u32);
        scene.vectorized_component(entity2, 16 as u32);

        assert_eq!(
            scene.get::<Vec<i32>>().iter().collect::<Vec<&Vec<i32>>>(),
            vec![&vec![34, 81]]
        );

        assert_eq!(
            scene.get::<Vec<f32>>().iter().collect::<Vec<&Vec<f32>>>(),
            vec![&vec![0.59]]
        );

        assert_eq!(
            scene.get::<Vec<u32>>().iter().collect::<Vec<&Vec<u32>>>(),
            vec![&vec![63, 16]]
        );
    }
}
