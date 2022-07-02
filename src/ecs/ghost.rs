use crate::ecs::{gizmos::PoolToolbox, ComponentPool, Scene};

impl Scene {
    pub fn toggle<G: 'static>(&mut self, entity: usize) {
        self.get_trait_mut::<G>().toggle_entity(entity);
    }

    pub fn enable<G: 'static>(&mut self, entity: usize) {
        self.get_trait_mut::<G>().enable_entity(entity);
    }

    pub fn disable<G: 'static>(&mut self, entity: usize) {
        self.get_trait_mut::<G>().disable_entity(entity);
    }
}

pub trait ComponentToggler {
    fn toggle_entity(&mut self, entity: usize);
    fn enable_entity(&mut self, entity: usize);
    fn disable_entity(&mut self, entity: usize);
    fn entity_in_scope(&mut self, entity: usize) -> bool;
    fn move_to_back(&mut self, entity: usize);
}

impl<G: 'static> ComponentToggler for ComponentPool<G> {
    fn toggle_entity(&mut self, entity: usize) {
        if self.entity_in_scope(entity) {
            self.disable_entity(entity);
        } else {
            self.enable_entity(entity);
        }
    }

    fn disable_entity(&mut self, entity: usize) {
        self.move_to_back(entity);
        self.num_components -= 1;
    }

    fn enable_entity(&mut self, entity: usize) {
        self.num_components += 1;
        self.move_to_back(entity);
    }

    /* Utility functions */

    fn entity_in_scope(&mut self, entity: usize) -> bool {
        (self.sparse_array[entity] as usize) < self.num_components
    }

    fn move_to_back(&mut self, entity: usize) {
        let component = self.sparse_array[entity] as usize;
        let component_destination = self.num_components - 1;

        let entity_destination = self.packed_array[component_destination];

        self.swap(entity, entity_destination, component, component_destination);
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::{ComponentPool, ComponentToggler, Scene};

    #[test]
    fn disabling_an_entities_component_makes_it_invisible() {
        let mut scene = Scene::new();

        let entity1 = scene.entity();
        let entity2 = scene.entity();
        let entity3 = scene.entity();

        scene.component(entity1, 32 as i32);
        scene.component(entity2, 64 as i32);
        scene.component(entity3, 128 as i32);

        scene.disable::<i32>(entity1);
        scene.disable::<i32>(entity2);

        assert_eq!(
            &ComponentPool {
                num_components: 1,

                sparse_array: vec! { 2, 1, 0 },
                packed_array: vec! { 2, 1, 0 },
                component_array: vec! { 128, 64, 32 },
            },
            scene.get::<i32>(),
        );
    }

    #[test]
    fn reenabling_disabled_entity_makes_it_reappear_into_scope() {
        let mut scene = Scene::new();

        let entity1 = scene.entity();
        let entity2 = scene.entity();
        let entity3 = scene.entity();

        scene.component(entity1, 32 as i32);
        scene.component(entity2, 64 as i32);
        scene.component(entity3, 128 as i32);

        scene.disable::<i32>(entity1);
        scene.disable::<i32>(entity2);

        scene.enable::<i32>(entity2);

        assert_eq!(
            &ComponentPool {
                num_components: 2,

                sparse_array: vec! { 2, 1, 0 },
                packed_array: vec! { 2, 1, 0 },
                component_array: vec! { 128, 64, 32 },
            },
            scene.get::<i32>(),
        );
    }

    #[test]
    fn iterating_over_component_pool_does_not_present_disabled_components() {
        let mut component_pool = ComponentPool::new_with_entity(2, 32);
        component_pool.assign_component(4, 64);
        component_pool.assign_component(5, 128);

        component_pool.disable_entity(2);

        assert_eq!(vec![&128, &64], component_pool.iter().collect::<Vec<_>>());
    }

    #[test]
    fn iterating_over_component_pool_presents_reenabled_components() {
        let mut component_pool = ComponentPool::new_with_entity(2, 32);
        component_pool.assign_component(4, 64);
        component_pool.assign_component(5, 128);

        component_pool.disable_entity(2);
        component_pool.disable_entity(5);

        component_pool.enable_entity(5);

        assert_eq!(vec![&64, &128], component_pool.iter().collect::<Vec<_>>());
    }
}
