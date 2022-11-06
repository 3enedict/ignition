use crate::life::{ComponentPool, Scene};

impl<P> Scene<P> {
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
}

impl<G> ComponentPool<G> {
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
    use crate::{
        life::{genesis::entity::EntityConstructor, ComponentPool, Scene},
        ComponentPools,
    };

    #[test]
    fn creating_an_entity_increments_an_id() {
        let mut scene: Scene<ComponentPools> = Scene::new();
        let mut entities: Vec<usize> = Vec::new();

        for _i in 0..10 {
            entities.push(scene.entity());
        }

        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], entities);
    }

    #[test]
    fn creating_an_entity_after_having_deleted_one_uses_recycled_id() {
        let mut scene: Scene<ComponentPools> = Scene::new();

        let entity = scene.entity();
        scene.delete(entity);

        assert_eq!(0, scene.entity());
    }

    #[test]
    fn prolonging_sparse_array_works_as_intended() {
        let mut sparse_array = vec![-1, -1, 0];
        ComponentPool::<i32>::prolong_sparse_array(5, &mut sparse_array);

        assert_eq!(vec![-1, -1, 0, -1, -1, -1], sparse_array,);
    }

    #[test]
    fn prolonging_sparse_array_with_a_smaller_than_length_id_does_nothing() {
        let mut sparse_array = vec![-1, -1, 0];
        ComponentPool::<i32>::prolong_sparse_array(2, &mut sparse_array);

        assert_eq!(vec![-1, -1, 0], sparse_array,);
    }

    #[test]
    fn adding_a_new_empty_entity_appends_minus_one_to_sparse_array() {
        let mut pool = ComponentPool::empty();
        pool.assign_component(3, 32);
        pool.create_empty_entity();

        assert_eq!(
            pool,
            ComponentPool {
                num_components: 1,

                sparse_array: vec![-1, -1, -1, 0, -1],
                packed_array: vec![3],
                component_array: vec![32],
            },
        );
    }
}
