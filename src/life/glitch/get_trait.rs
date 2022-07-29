#[cfg(test)]
mod tests {
    use crate::life::{glitch::LifeError, Scene};

    #[test]
    fn requesting_for_non_existing_component_pool_returns_error() {
        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, 1 as i32);

        assert_err!(scene.get_trait::<f32>(), LifeError::NoComponentPool("f32"));

        assert_err!(
            scene.get_trait_mut::<f32>(),
            LifeError::NoComponentPool("f32")
        );
    }
}
