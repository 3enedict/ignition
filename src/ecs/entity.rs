use crate::ecs::Scene;

impl Scene {
    pub fn entity(&mut self) -> usize {
        if self.available_entities.len() == 1 {
            self.generate_new_entity()
        } else {
            self.use_recycled_entity()
        }
    }

    pub fn delete(&mut self, entity: usize) {
        self.available_entities.push(entity);
        self.delete_entity_from_each_component_pool(entity);
    }

    /* Utility functions */

    pub fn get_current_entity(&self) -> usize {
        self.available_entities[self.available_entities.len() - 1]
    }

    pub fn generate_new_entity(&mut self) -> usize {
        let id = self.available_entities[0];
        self.available_entities[0] += 1;

        id
    }

    pub fn use_recycled_entity(&mut self) -> usize {
        self.available_entities.pop().unwrap()
    }

    pub fn delete_entity_from_each_component_pool(&mut self, entity: usize) {
        for component_pool in self.component_pools.iter_mut() {
            component_pool.delete_entity(entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::Scene;

    #[test]
    fn generated_entity_is_correct() {
        let mut scene = Scene::new();

        assert_eq!(0, scene.generate_new_entity());
    }

    #[test]
    fn entity_generation_updates_available_entities() {
        let mut scene = Scene::new();
        scene.generate_new_entity();

        assert_eq!(1, scene.available_entities[0]);
    }

    #[test]
    fn deleted_entity_is_added_to_the_list_of_available_entities_for_recycling() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.delete(entity);

        assert_eq!(vec![1, 0], scene.available_entities);
    }

    #[test]
    fn recycle_entities_when_they_exist() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.delete(entity);
        let recycled_entity = scene.entity();

        assert_eq!(0, recycled_entity);
    }
}
