use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LifeError {
    #[error("There's no component pool for : {0}")]
    NoComponentPool(String),
    #[error("Downcasting from Box<dyn ComponentPoolTrait> to ComponentPool<G> failed for : {0}")]
    Downcast(String),
    #[error("Entity {1} is out of scope for : {0}")]
    EntityOutOfScope(String, usize),
}
