use crate::ecs::Scene;

impl Scene {
    pub fn enable<G: 'static>(&mut self, entity: usize) {
        self.get_component_pool_trait_mut::<G>()
            .enable_entity(entity);
    }

    pub fn disable<G: 'static>(&mut self, entity: usize) {
        self.get_component_pool_trait_mut::<G>()
            .disable_entity(entity);
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::component::component_pool::ComponentPool;
    use crate::ecs::Scene;

    #[test]
    fn disable_an_entity_s_component_makes_it_invisible() {
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

                sparse_array: vec! { 2, 0, 1 },
                packed_array: vec! { 2, 1, 0 },
                component_array: vec! { 128, 64, 32 },
            },
            scene.get_component_pool::<i32>(),
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
            scene.get_component_pool::<i32>(),
        );
    }
}
