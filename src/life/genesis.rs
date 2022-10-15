pub mod component;
pub mod entity;

use crate::life::{ComponentPool, Scene};

impl Scene {
    pub fn new() -> Self {
        Self {
            available_entities: vec![0],
            component_pools: Vec::new(),
        }
    }
}

impl<G> ComponentPool<G> {
    pub fn empty() -> Self {
        Self {
            num_components: 0,

            sparse_array: Vec::new(),
            packed_array: Vec::new(),
            component_array: Vec::new(),
        }
    }
}
