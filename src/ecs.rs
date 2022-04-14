use crate::core::rendering::vertex_buffer::Vertex;

pub struct Vertices {
    pub vertices: Vec<Vertex>,
}

pub trait ComponentPoolTrait {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
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

    pub fn entity(&mut self) -> i32 {
        self.entity_count += 1;

        for component_pool in self.component_pools.iter_mut() {
            component_pool.push_none();
        }

        self.entity_count
    }

    pub fn component<G: 'static>(&mut self, entity: i32, component: G) {
        for component_pool in self.component_pools.iter_mut() {
            if let Some(component_pool) = component_pool
                .as_any_mut()
                .downcast_mut::<ComponentPool<G>>()
            {
                component_pool.components.insert(entity as usize, component);

                return;
            }
        }

        let mut new_component_pool_vec = Vec::with_capacity(entity as usize);
        new_component_pool_vec.insert(entity as usize, component);

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
    fn test_add_new_entity() {
        let mut scene = IgnitionScene::new();

        let entity1 = scene.entity();
        scene.component(entity1, Pos { x: 1, y: -3 });
        scene.component(entity1, Vel { speed: 30 });

        let entity2 = scene.entity();
        scene.component(entity2, Pos { x: 5, y: 2 });
        scene.component(entity2, Vel { speed: 3 });

        assert_eq!(
            &mut vec! { Pos { x: 1, y: -3 }, Pos { x: 5, y: 2 } },
            scene.get_components(0)
        );

        assert_eq!(
            &mut vec! { Vel { speed: 30 }, Vel { speed: 3 } },
            scene.get_components(1)
        );
    }
}
