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
    pub components: Vec<G>,
}

impl<G: 'static> ComponentPoolTrait for ComponentPool<G> {
    fn push_none(&mut self) {
        self.components.reserve(1);
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
    pub entity_count: i32,
    pub component_pools: Vec<Box<dyn ComponentPoolTrait>>,
}

impl IgnitionScene {
    pub fn new() -> Self {
        Self {
            entity_count: -1,
            component_pools: Vec::new(),
        }
    }

    pub fn entity(&mut self) -> Entity {
        self.entity_count += 1;

        for component_pool in self.component_pools.iter_mut() {
            component_pool.push_none();
        }

        Entity {
            id: self.entity_count as usize,
            bitmask: BitSet::new(),
        }
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
                component_pool.components.insert(entity.id, component);
                entity.bitmask.insert(i);

                return;
            }
        }

        entity.bitmask.insert(self.component_pools.len());

        let mut new_component_pool_vec = Vec::with_capacity(entity.id);
        new_component_pool_vec.insert(entity.id, component);

        self.component_pools.push(Box::new(ComponentPool {
            components: new_component_pool_vec,
        }));
    }

    pub fn get_components<G: 'static>(&mut self, index: usize) -> &mut Vec<G> {
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

    #[derive(Debug, Eq, PartialEq)]
    struct Pos {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Eq, PartialEq)]
    struct Vel {
        speed: u32,
    }

    #[test]
    fn test_adding_components() {
        let mut scene = IgnitionScene::new();

        let mut entity1 = scene.entity();
        scene.component(&mut entity1, Pos { x: 1, y: -3 });
        scene.component(&mut entity1, Vel { speed: 30 });

        let mut entity2 = scene.entity();
        scene.component(&mut entity2, Pos { x: 5, y: 2 });
        scene.component(&mut entity2, Vel { speed: 3 });

        assert_eq!(
            &mut vec! { Pos { x: 1, y: -3 }, Pos { x: 5, y: 2 } },
            scene.get_components(0)
        );

        assert_eq!(
            &mut vec! { Vel { speed: 30 }, Vel { speed: 3 } },
            scene.get_components(1)
        );
    }

    #[test]
    fn test_entity_bitmasks() {
        let mut scene = IgnitionScene::new();

        let mut entity1 = scene.entity();
        scene.component(&mut entity1, Pos { x: 1, y: -3 });
        scene.component(&mut entity1, Vel { speed: 30 });

        let mut entity2 = scene.entity();
        scene.component(&mut entity2, Vel { speed: 3 });

        let mut expected_result1 = BitSet::new();
        expected_result1.insert(0);
        expected_result1.insert(1);
        assert_eq!(entity1.bitmask, expected_result1);

        let mut expected_result2 = BitSet::new();
        expected_result2.insert(1);
        assert_eq!(entity2.bitmask, expected_result2);
    }
}
