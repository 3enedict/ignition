use crate::life::{Component, ComponentPool, PoolToolbox, Scene};

impl Scene {
    pub fn component<G: 'static + Component>(&mut self, entity: usize, component: G) {
        if self.component_pool_exists::<G>() {
            self.assign_component(entity, component);
        } else {
            self.new_component_pool(entity, component);
        }
    }

    pub fn assign_component<G: 'static + Component>(&mut self, entity: usize, component: G) {
        unwrap!(self.get_mut::<G>()).assign_component(entity, component);
    }

    pub fn new_component_pool<G: 'static + Component>(&mut self, entity: usize, component: G) {
        let mut component_pool = Box::new(ComponentPool::empty());
        component_pool.assign_component(entity, component);

        self.component_pools.resize_with(G::id() + 1, || None);
        self.component_pools[G::id()] = Some(component_pool);
    }
}

impl<G: 'static> ComponentPool<G> {
    pub fn assign_component(&mut self, entity: usize, component: G) {
        if self.has_component(entity) {
            *unwrap!(self.get_mut(entity)) = component;
        } else {
            Self::add_entity_to_sparse_array(entity, self.num_components, &mut self.sparse_array);

            self.packed_array.push(entity);
            self.component_array.push(component);
            self.num_components += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use component_derive::Component;

    use crate::life::{Component, Scene};

    #[derive(Component, Debug, PartialEq)]
    struct Number {
        num: i32,
    }

    #[test]
    fn creating_new_component_pool_updates_scene() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, Number { num: 34 });

        assert_eq!(
            scene
                .get::<Number>()
                .unwrap()
                .iter()
                .collect::<Vec<&Number>>(),
            vec![&Number { num: 34 }]
        );
    }

    #[test]
    fn assigning_component_in_previously_created_component_pool_updates_scene() {
        let mut scene = Scene::new();

        let entity1 = scene.entity();
        scene.component(entity1, Number { num: 34 });

        let entity2 = scene.entity();
        scene.component(entity2, Number { num: 25 });

        assert_eq!(
            scene
                .get::<Number>()
                .unwrap()
                .iter()
                .collect::<Vec<&Number>>(),
            vec![&Number { num: 34 }, &Number { num: 25 }]
        );
    }

    #[test]
    fn assigning_already_existing_component_modifies_current_component() {
        let mut scene = Scene::new();
        let entity = scene.entity();

        scene.component(entity, Number { num: 34 });
        scene.component(entity, Number { num: 25 });

        assert_eq!(
            scene
                .get::<Number>()
                .unwrap()
                .iter()
                .collect::<Vec<&Number>>(),
            vec![&Number { num: 25 }]
        );
    }
}
