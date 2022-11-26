use crate::life::{Component, ComponentPool, PoolToolbox, Scene};

impl<P> Scene<P> {
    pub fn component<G: 'static + Component<P>>(&mut self, entity: usize, component: G) {
        self.get_mut::<G>().assign_component(entity, component);
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
    use crate::{life::Scene, ComponentPools, Number};

    #[test]
    fn creating_new_component_pool_updates_scene() {
        let mut scene: Scene<ComponentPools> = Scene::new();

        let entity = scene.entity();
        scene.component(entity, Number { num: 4 });

        assert_eq!(
            scene.get::<Number>().iter().collect::<Vec<&Number>>(),
            vec![&Number { num: 4 }]
        );
    }

    #[test]
    fn assigning_component_in_previously_created_component_pool_updates_scene() {
        let mut scene: Scene<ComponentPools> = Scene::new();

        let entity1 = scene.entity();
        scene.component(entity1, Number { num: 34 });

        let entity2 = scene.entity();
        scene.component(entity2, Number { num: 25 });

        assert_eq!(
            scene.get::<Number>().iter().collect::<Vec<&Number>>(),
            vec![&Number { num: 34 }, &Number { num: 25 }]
        );
    }

    #[test]
    fn assigning_already_existing_component_modifies_current_component() {
        let mut scene: Scene<ComponentPools> = Scene::new();
        let entity = scene.entity();

        scene.component(entity, Number { num: 34 });
        scene.component(entity, Number { num: 25 });

        assert_eq!(
            scene.get::<Number>().iter().collect::<Vec<&Number>>(),
            vec![&Number { num: 25 }]
        );
    }
}
