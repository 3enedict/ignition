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
    use bit_set::BitSet;

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
    fn test_adding_components_pos() {
        let (mut scene, _entity1, _entity2, _entity3) =
            init_three_entities_with_different_components();

        assert_eq!(
            &mut vec! { None, None, Some ( Pos { x: 1, y: -3 } ) },
            scene.get_components(1)
        );
    }

    #[test]
    fn test_adding_components_vel() {
        let (mut scene, _entity1, _entity2, _entity3) =
            init_three_entities_with_different_components();

        assert_eq!(
            &mut vec! { Some ( Vel { speed: 286 } ), None, Some ( Vel { speed: 30 } ) },
            scene.get_components(0)
        );
    }

    #[test]
    fn test_entity_bitmasks_entity1() {
        let (_scene, entity1, _entity2, _entity3) = init_three_entities_with_different_components();

        let mut expected_result1 = BitSet::new();
        expected_result1.insert(0);
        assert_eq!(entity1.bitmask, expected_result1);
    }

    #[test]
    fn test_entity_bitmasks_entity2() {
        let (_scene, _entity1, entity2, _entity3) = init_three_entities_with_different_components();

        let expected_result2 = BitSet::new();
        assert_eq!(entity2.bitmask, expected_result2);
    }

    #[test]
    fn test_entity_bitmasks_entity3() {
        let (_scene, _entity1, _entity2, entity3) = init_three_entities_with_different_components();

        let mut expected_result3 = BitSet::new();
        expected_result3.insert(0);
        expected_result3.insert(1);
        assert_eq!(entity3.bitmask, expected_result3);
    }

    #[test]
    fn test_deleting_then_adding_an_entity() {
        let (mut scene, entity1, _entity2, _entity3) =
            init_three_entities_with_different_components();

        scene.delete_entity(entity1);

        let mut entity4 = scene.entity();
        scene.component(&mut entity4, Pos { x: 43, y: 96 });

        assert_eq!(
            &mut vec! { Some ( Pos { x: 43, y: 96 } ), None, Some ( Pos { x: 1, y: -3 } ) },
            scene.get_components(1)
        );
    }
}
