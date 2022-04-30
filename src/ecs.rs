pub mod entity;

pub mod component;
use component::ComponentPoolTrait;

pub struct IgnitionScene {
    pub entity_count: usize,
    pub available_entities: Vec<usize>,

    pub component_pools: Vec<Box<dyn ComponentPoolTrait>>,
}

impl IgnitionScene {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            available_entities: vec![0],

            component_pools: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::entity::Entity;
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

    fn init_three_entities_with_different_components() -> (IgnitionScene, Entity, Entity, Entity) {
        let mut scene = IgnitionScene::new();

        let mut entity1 = scene.entity();
        scene.component(&mut entity1, Vel { speed: 286 });

        let entity2 = scene.entity();

        let mut entity3 = scene.entity();
        scene.component(&mut entity3, Pos { x: 1, y: -3 });
        scene.component(&mut entity3, Vel { speed: 30 });

        (scene, entity1, entity2, entity3)
    }

    #[test]
    fn add_components_pos_sparse_array() {
        let (mut scene, _entity1, _entity2, _entity3) =
            init_three_entities_with_different_components();

        assert_eq!(&mut vec! { -1, -1, 0 }, scene.get_sparse_array::<Pos>(1));
    }

    #[test]
    fn add_components_pos_packed_array() {
        let (mut scene, _entity1, _entity2, _entity3) =
            init_three_entities_with_different_components();

        assert_eq!(&mut vec! { 2 }, scene.get_packed_array::<Pos>(1));
    }

    #[test]
    fn add_components_pos_component_array() {
        let (mut scene, _entity1, _entity2, _entity3) =
            init_three_entities_with_different_components();

        assert_eq!(
            &mut vec! { Pos { x: 1, y: -3 } },
            scene.get_component_array::<Pos>(1)
        );
    }

    #[test]
    fn add_components_vel_sparse_array() {
        let (mut scene, _entity1, _entity2, _entity3) =
            init_three_entities_with_different_components();

        assert_eq!(&mut vec! { 0, -1, 1 }, scene.get_sparse_array::<Vel>(0));
    }

    #[test]
    fn add_components_vel_packed_array() {
        let (mut scene, _entity1, _entity2, _entity3) =
            init_three_entities_with_different_components();

        assert_eq!(&mut vec! { 0, 2 }, scene.get_packed_array::<Vel>(0));
    }

    #[test]
    fn add_components_vel_component_array() {
        let (mut scene, _entity1, _entity2, _entity3) =
            init_three_entities_with_different_components();

        assert_eq!(
            &mut vec! { Vel { speed: 286 }, Vel { speed: 30 } },
            scene.get_component_array::<Vel>(0)
        );
    }
}
