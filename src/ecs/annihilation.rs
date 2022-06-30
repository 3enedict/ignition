use crate::ecs::{ComponentPool, Scene};

impl Scene {
    pub fn delete(&mut self, entity: usize) {
        self.available_entities.push(entity);
        self.delete_entity_from_each_component_pool(entity);
    }

    pub fn delete_entity_from_each_component_pool(&mut self, entity: usize) {
        for (_type_id, component_pool) in self.component_pools.iter_mut() {
            component_pool.delete_entity(entity);
        }
    }
}

pub trait EntityDestructor {
    fn delete_entity(&mut self, entity: usize);
}

impl<G: 'static> EntityDestructor for ComponentPool<G> {
    fn delete_entity(&mut self, entity: usize) {
        let index = self.sparse_array[entity];

        if index != -1 {
            self.num_components -= 1;

            self.packed_array.swap_remove(index as usize);
            self.component_array.swap_remove(index as usize);

            let last_index = self.sparse_array.len() - 1;
            self.sparse_array[last_index] = self.sparse_array[entity];
            self.sparse_array[entity] = -1;

            self.packed_array[index as usize] = last_index;
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::ecs::{ComponentPool, EntityDestructor, Scene};

    #[test]
    fn deleting_an_entity_adds_its_id_to_the_list_of_available_entities() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.delete(entity);

        assert_eq!(vec![1, 0], scene.available_entities);
    }

    #[test]
    fn deleting_an_entity_is_correctly_reflected_in_sparse_array() {
        let mut component_pool = ComponentPool::new_with_entity(2, 32);
        component_pool.assign_component(4, 64);
        component_pool.assign_component(5, 128);

        component_pool.delete_entity(2);

        assert_eq!(
            ComponentPool {
                num_components: 2,

                sparse_array: vec! { -1, -1, -1, -1, 1, 0 },
                packed_array: vec! { 5, 4 },
                component_array: vec! { 128, 64 },
            },
            component_pool,
        );
    }
}
