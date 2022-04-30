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

impl IgnitionScene {
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
