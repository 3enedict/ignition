use thiserror::Error;

#[cfg(test)]
macro_rules! assert_err {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            Err(e) => assert_eq!(e, $($pattern)+),
            Ok(_) => panic!("`{}` did not return error as expected", stringify!($expression)),
        }
    }
}

mod get;
mod get_component;
mod get_trait;

#[derive(Error, Debug, PartialEq)]
pub enum LifeError<'a> {
    #[error("There's no component pool for : {0}")]
    NoComponentPool(&'a str),
    #[error("Downcasting from Box<dyn ComponentPoolTrait> to ComponentPool<G> failed for : {0}")]
    Downcast(&'a str),
    #[error("Entity {1} is out of scope for : {0}")]
    EntityOutOfScope(&'a str, usize),
    #[error("Entity {1} does not have any component bound to it for : {0}")]
    EntityNotBoundToComponent(&'a str, usize),
}
