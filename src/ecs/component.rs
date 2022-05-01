use std::any::TypeId;

use crate::ecs::entity::Entity;
use crate::ecs::IgnitionScene;

pub trait ComponentPoolTrait {
    fn push_none(&mut self);

    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn component_as_any(&self, index: usize) -> &dyn std::any::Any;
    fn component_as_any_mut(&mut self, index: usize) -> &mut dyn std::any::Any;
}

pub struct ComponentPool<G> {
    pub num_components: i32,

    pub sparse_array: Vec<i32>,
    pub packed_array: Vec<i32>,
    pub component_array: Vec<G>,
}

impl<G: 'static> ComponentPoolTrait for ComponentPool<G> {
    fn push_none(&mut self) {
        self.sparse_array.push(-1);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn component_as_any(&self, index: usize) -> &dyn std::any::Any {
        self.component_array.get(index).unwrap() as &dyn std::any::Any
    }

    fn component_as_any_mut(&mut self, index: usize) -> &mut dyn std::any::Any {
        self.component_array.get_mut(index).unwrap() as &mut dyn std::any::Any
    }
}

impl IgnitionScene {
    pub fn component<G: 'static>(&mut self, entity: &mut Entity, component: G) {
        if self.component_indices.contains_key(&TypeId::of::<G>()) {
            let component_pool = self.get_component_pool::<G>();

            component_pool.sparse_array[entity.id] = component_pool.num_components;

            component_pool.packed_array.push(entity.id as i32);
            component_pool.component_array.push(component);

            component_pool.num_components += 1;

            return;
        }

        self.component_indices
            .insert(TypeId::of::<G>(), self.component_pools.len());

        let mut sparse_array = Vec::with_capacity(entity.id);
        sparse_array.resize_with(entity.id, || -1);
        sparse_array.push(0);

        let packed_array = vec![entity.id as i32];
        let component_array = vec![component];

        self.component_pools.push(Box::new(ComponentPool {
            num_components: 1,

            sparse_array,
            packed_array,
            component_array,
        }));
    }

    pub fn get_component_pool<G: 'static>(&mut self) -> &mut ComponentPool<G> {
        self.component_pools
            .get_mut(*self.component_indices.get(&TypeId::of::<G>()).unwrap())
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentPool<G>>()
            .unwrap()
    }
}
