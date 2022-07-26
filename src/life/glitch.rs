use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LifeError {
    #[error("There's no component pool for : {0}")]
    NoComponentPool(String),
}
