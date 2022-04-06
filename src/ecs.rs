use std::any::Any;
use std::borrow::Borrow;

use crate::core::rendering::vertex_buffer::Vertex;

pub struct Vertices {
    pub vertices: Vec<Vertex>,
}

pub struct ComponentPool {
    components: Vec<Box<dyn Any>>,
}

pub struct IgnitionScene {
    pub entity_count: usize,
    pub component_pools: Vec<ComponentPool>,
}

impl IgnitionScene {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            component_pools: Vec::new(),
        }
    }

    pub fn entity(&mut self) -> usize {
        self.entity_count += 1;

        self.entity_count
    }

    pub fn component(&mut self, entity: usize, component: Box<dyn Any>) {
        let mut component_index = -1;

        let mut counter = 0;
        for component_pool in self.component_pools.iter_mut() {
            if component_pool.components[0].type_id() == component.type_id() {
                component_index = counter;
            }

            counter += 1;
        }

        if component_index == -1 {
            self.component_pools.push(ComponentPool {
                components: vec![component],
            })
        } else {
            self.component_pools[component_index as usize]
                .components
                .push(component);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Pos {
        x: i32,
        y: i32,
    }

    struct Vel {
        speed: u32,
    }

    #[test]
    fn test_add_new_entity() {
        let mut scene = IgnitionScene::new();

        let entity1 = scene.entity();
        scene.component(entity1, Box::new(Pos { x: 1, y: -3 }));
        scene.component(entity1, Box::new(Vel { speed: 30 }));

        let entity2 = scene.entity();
        scene.component(entity2, Box::new(Pos { x: 5, y: 2 }));
        scene.component(entity2, Box::new(Vel { speed: 3 }));

        assert_eq!(
            scene.component_pools[0].components[0]
                .downcast::<Pos>()
                .unwrap()
                .x,
            1
        );
        assert_eq!(
            scene.component_pools[0].components[0]
                .downcast::<Pos>()
                .unwrap()
                .y,
            -3
        );
        assert_eq!(
            scene.component_pools[0].components[1]
                .downcast::<Pos>()
                .unwrap()
                .x,
            5
        );
        assert_eq!(
            scene.component_pools[0].components[1]
                .downcast::<Pos>()
                .unwrap()
                .x,
            5
        );

        assert_eq!(
            scene.component_pools[1].components[0]
                .downcast::<Vel>()
                .unwrap()
                .speed,
            30
        );
        assert_eq!(
            scene.component_pools[1].components[1]
                .downcast::<Vel>()
                .unwrap()
                .speed,
            3
        );
    }
}
