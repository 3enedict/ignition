#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use crate::life::{glitch::LifeError, ComponentPool, Scene};

    #[test]
    fn getting_component_from_entity_thats_out_of_scope_return_error() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        assert_err!(
            scene.get_component::<i32>(3),
            LifeError::EntityOutOfScope("i32", 3)
        );

        assert_err!(
            scene.get_component_mut::<i32>(3),
            LifeError::EntityOutOfScope("i32", 3)
        );
    }

    #[test]
    fn getting_component_from_entity_that_isnt_bound_to_any_component_returns_error() {
        let mut scene = Scene::new();

        let type_id = TypeId::of::<i32>();
        let component_pool = Box::new(ComponentPool {
            num_components: 1,

            sparse_array: vec![-1, -1, -1, 1],
            packed_array: vec![3],
            component_array: vec![32],
        });
        scene.component_pools.insert(type_id, component_pool);

        assert_err!(
            scene.get_component::<i32>(3),
            LifeError::EntityNotBoundToComponent("i32", 3)
        );

        assert_err!(
            scene.get_component_mut::<i32>(3),
            LifeError::EntityNotBoundToComponent("i32", 3)
        );
    }

    #[test]
    fn downcast_error_is_correctly_propagated_at_get_component() {
        let mut scene = Scene::new();

        let type_id = TypeId::of::<f32>();
        let component_pool = Box::new(ComponentPool::new_with_entity(1, 32 as i32));
        scene.component_pools.insert(type_id, component_pool);

        assert_err!(scene.get_component::<f32>(3), LifeError::Downcast("f32"));

        assert_err!(
            scene.get_component_mut::<f32>(3),
            LifeError::Downcast("f32")
        );
    }
}
