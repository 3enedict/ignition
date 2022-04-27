use crate::core::rendering::vertex_buffer::Vertex;
use bit_set::BitSet;

pub struct Entity {
    pub id: usize,
    pub bitmask: BitSet,
}

pub struct Vertices {
    pub vertices: Vec<Vertex>,
}

pub trait ComponentPoolTrait {
    fn push_none(&mut self);

    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn component_as_any(&self, index: usize) -> &dyn std::any::Any;
    fn component_as_any_mut(&mut self, index: usize) -> &mut dyn std::any::Any;
}

pub struct ComponentPool<G> {
    pub components: Vec<Option<G>>,
}

impl<G: 'static> ComponentPoolTrait for ComponentPool<G> {
    fn push_none(&mut self) {
        self.components.push(None);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn component_as_any(&self, index: usize) -> &dyn std::any::Any {
        self.components.get(index).unwrap() as &dyn std::any::Any
    }

    fn component_as_any_mut(&mut self, index: usize) -> &mut dyn std::any::Any {
        self.components.get_mut(index).unwrap() as &mut dyn std::any::Any
    }
}

pub struct IgnitionScene {
    pub entity_count: usize,
    pub component_pools: Vec<Box<dyn ComponentPoolTrait>>,
}

impl IgnitionScene {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            component_pools: Vec::new(),
        }
    }

    pub fn entity(&mut self) -> Entity {
        for component_pool in self.component_pools.iter_mut() {
            component_pool.push_none();
        }

        let new_entity = Entity {
            id: self.entity_count,
            bitmask: BitSet::new(),
        };

        self.entity_count += 1;

        new_entity
    }

    pub fn component<G: 'static>(&mut self, entity: &mut Entity, component: G) {
        for i in 0..self.component_pools.len() {
            if let Some(component_pool) = self
                .component_pools
                .get_mut(i)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<ComponentPool<G>>()
            {
                component_pool.components[entity.id] = Some(component);
                entity.bitmask.insert(i);

                return;
            }
        }

        entity.bitmask.insert(self.component_pools.len());

        let mut new_component_pool_vec = Vec::with_capacity(entity.id);
        new_component_pool_vec.resize_with(entity.id, || None);
        new_component_pool_vec.push(Some(component));

        self.component_pools.push(Box::new(ComponentPool {
            components: new_component_pool_vec,
        }));
    }

    pub fn get_components<G: 'static>(&mut self, index: usize) -> &mut Vec<Option<G>> {
        &mut self
            .component_pools
            .get_mut(index)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentPool<G>>()
            .unwrap()
            .components
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
