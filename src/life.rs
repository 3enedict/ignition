use std::any::TypeId;
use std::collections::HashMap;

use annihilation::EntityDestructor;
use genesis::EntityConstructor;
use ghost::ComponentToggler;
use gizmos::PoolToolbox;

pub mod abduction;
pub mod annihilation;
pub mod genesis;
pub mod ghost;
pub mod gizmos;

pub struct Scene {
    pub available_entities: Vec<usize>,
    pub component_pools: HashMap<TypeId, Box<dyn ComponentPoolTrait>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ComponentPool<G> {
    pub num_components: usize,

    pub sparse_array: Vec<i32>,
    pub packed_array: Vec<usize>,
    pub component_array: Vec<G>,
}

pub trait ComponentPoolTrait:
    EntityConstructor + EntityDestructor + ComponentToggler + PoolToolbox
{
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
