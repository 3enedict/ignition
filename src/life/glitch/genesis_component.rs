#[cfg(test)]
mod tests {
    use log::Level;
    extern crate testing_logger;

    use crate::life::{ComponentPool, Scene};

    #[test]
    fn error_in_vectorized_component_is_logged() {
        testing_logger::setup();

        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.component(entity, vec![2 as i32]);

        scene.get_mut::<Vec<i32>>().unwrap().sparse_array[0] = 1;

        scene.vectorized_component(entity, 3 as i32);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, Level::Warn);

            assert_contains!(
                captured_logs[0].body,
                "Entity 0 is bound to a non existing component for"
            );

            assert_contains!(captured_logs[0].body, "i32"); // The reason why i'm pattern matching against i32 instead of using type_name::<i32>() is because the latter isn't reliable as said in the docs
        });
    }

    #[test]
    fn error_in_assign_component_is_logged() {
        testing_logger::setup();

        let mut scene = Scene::new();

        let entity = scene.entity();
        scene.assign_component(entity, 3 as i32);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, Level::Warn);

            assert_contains!(captured_logs[0].body, "There's no component pool for");

            assert_contains!(captured_logs[0].body, "i32"); // The reason why i'm pattern matching against i32 instead of using type_name::<i32>() is because the latter isn't reliable as said in the docs
        });
    }

    #[test]
    fn error_in_component_pool_assignement_is_logged() {
        testing_logger::setup();

        let mut pool = ComponentPool::new_with_entity(2, 3 as i32);

        pool.sparse_array[2] = 1;
        pool.assign_component(2, 2 as i32);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, Level::Warn);

            assert_contains!(
                captured_logs[0].body,
                "Entity 2 is bound to a non existing component for"
            );

            assert_contains!(captured_logs[0].body, "i32"); // The reason why i'm pattern matching against i32 instead of using type_name::<i32>() is because the latter isn't reliable as said in the docs
        });
    }
}
