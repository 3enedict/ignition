use std::any::TypeId;
use std::collections::HashMap;

pub mod entity;

pub mod component;
use component::component_pool_trait::ComponentPoolTrait;

pub struct IgnitionScene {
    pub entity_count: usize,
    pub available_entities: Vec<usize>,

    pub component_indices: HashMap<TypeId, usize>,
    pub component_pools: Vec<Box<dyn ComponentPoolTrait>>,
}

impl IgnitionScene {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            available_entities: Vec::new(),

            component_indices: HashMap::new(),
            component_pools: Vec::new(),
        }
    }
}

/* TESTS */

#[cfg(test)]
mod tests {
    use crate::ecs::component::component_pool::ComponentPool;
    use crate::ecs::IgnitionScene;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Pos {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Vel {
        speed: u32,
    }

    fn init_three_entities() -> (IgnitionScene, usize, usize, usize) {
        let mut scene = IgnitionScene::new();

        let entity1 = scene.entity();
        scene.component(entity1, Vel { speed: 286 });

        let entity2 = scene.entity();

        let entity3 = scene.entity();
        scene.component(entity3, Pos { x: 1, y: -3 });
        scene.component(entity3, Vel { speed: 30 });

        (scene, entity1, entity2, entity3)
    }

    #[test]
    fn add_component() {
        let (mut scene, _entity1, _entity2, _entity3) = init_three_entities();

        assert_eq!(
            &mut ComponentPool {
                num_components: 1,

                sparse_array: vec! { -1, -1, 0 },
                packed_array: vec! { 2 },
                component_array: vec! { Pos { x: 1, y: -3 } },
            },
            scene.get_component_pool::<Pos>()
        );

        assert_eq!(
            &mut ComponentPool {
                num_components: 2,

                sparse_array: vec! { 0, -1, 1 },
                packed_array: vec! { 0, 2 },
                component_array: vec! { Vel { speed: 286 }, Vel { speed: 30 } },
            },
            scene.get_component_pool::<Vel>()
        );
    }

    #[test]
    fn recycle_entity() {
        let (mut scene, entity1, _entity2, _entity3) = init_three_entities();

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
            scene.get_component_pool::<Pos>()
        );

        assert_eq!(
            &mut ComponentPool {
                num_components: 1,

                sparse_array: vec! { -1, -1, 0 },
                packed_array: vec! { 2 },
                component_array: vec! { Vel { speed: 30 } },
            },
            scene.get_component_pool::<Vel>()
        );
    }
}
