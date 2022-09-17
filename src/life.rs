use component_derive::Component;

pub mod genesis;

pub trait Component {
    fn id() -> usize;
}

#[derive(Component)]
struct Test {}
