#[cfg(test)]
mod tests {
    /*
    use std::any::TypeId;

    use crate::life::{glitch::LifeError, ComponentPool, Scene};

    #[test]
    fn unable_to_downcast_component_pool_trait_returns_error() {
        let mut scene = Scene::new();

        let type_id = TypeId::of::<f32>();
        let component_pool = Box::new(ComponentPool::new_with_entity(1, 32 as i32));
        scene.component_pools.insert(type_id, component_pool);

        assert_err!(scene.get::<f32>(), LifeError::Downcast("f32"));
        assert_err!(scene.get_mut::<f32>(), LifeError::Downcast("f32"));
    }

    #[test]
    fn no_component_pool_error_is_correctly_propagated_at_get() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        assert_err!(scene.get::<f32>(), LifeError::NoComponentPool("f32"));
        assert_err!(scene.get_mut::<f32>(), LifeError::NoComponentPool("f32"));
    }
    */
}
