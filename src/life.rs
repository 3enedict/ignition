use annihilation::EntityDestructor;
use genesis::entity::EntityConstructor;
use ghost::ComponentToggler;
use gizmos::PoolToolbox;

pub mod abduction;
pub mod annihilation;
pub mod genesis;
pub mod ghost;
pub mod gizmos;
pub mod glitch;

use component::Component;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct Number {
    num: i32,
}

pub struct Scene<P> {
    pub available_entities: Vec<usize>,
    pub component_pools: P,
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

pub trait Component<G> {
    fn get_from(component_pools: &G) -> &ComponentPool<Self>
    where
        Self: Sized;

    fn get_mut_from(component_pools: &mut G) -> &mut ComponentPool<Self>
    where
        Self: Sized;
}

pub trait ComponentPoolsTrait {
    fn new() -> Self;
    fn delete_entity(&mut self, entity: usize);
}
