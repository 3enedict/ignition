use thiserror::Error;

#[cfg(test)]
macro_rules! assert_err {
    ($expression:expr, $($error:tt)+) => {
        match $expression {
            Err(e) => assert_eq!(e, $($error)+),
            Ok(_) => panic!("`{}` did not return error as expected", stringify!($expression)),
        }
    }
}

#[cfg(test)]
macro_rules! assert_contains {
    ($error:expr, $msg:literal) => {
        if !$error.contains($msg) {
            panic!("Expected to find `{}` in `{}` ", $msg, $error);
        }
    };
}

mod get;
mod get_component;
mod get_trait;

mod genesis_component;

#[derive(Error, Debug, PartialEq)]
pub enum LifeError {
    #[error("There's no component pool for : {0}")]
    NoComponentPool(&'static str),
    #[error("Downcasting from Box<dyn ComponentPoolTrait> to ComponentPool<G> failed for : {0}. Note: this is supposed to be impossible, so there's probably a rather large bug in ignition")]
    Downcast(&'static str),
    #[error("Entity {1} has not been found for : {0}. Perhaps it is out of scope of `ComponentPool<{0}>.sparse_array`.")]
    EntityNotFound(&'static str, usize),
    #[error("Component {1} has not been found for : {0}. Perhaps it is out of scope of `ComponentPool<{0}>.packed_array`.")]
    ComponentNotFound(&'static str, usize),
    #[error("Entity {1} does not have any component bound to it for : {0}.")]
    EntityNotBoundToComponent(&'static str, usize),
    #[error("Entity {1} is bound to a non existing component for : {0}. Note: this is supposed to be impossible, so there's probably a rather large bug in ignition")]
    EntityBoundToNonExistingComponent(&'static str, usize),
}
