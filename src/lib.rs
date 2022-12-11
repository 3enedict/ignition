macro_rules! unwrap {
    ($expression:expr) => {
        match $expression {
            Ok(value) => value,
            Err(e) => {
                log::warn!("{}", e);
                return;
            }
        }
    };
}

extern crate component;
extern crate engine;

pub mod liberty;
pub mod life;
pub mod manifestation;

pub mod prelude {
    pub use component::Component;
    pub use wgpu::include_wgsl;

    pub use crate::{life::Component, Engine};
}

use crate::{
    liberty::RuntimeConfiguration,
    life::{annihilation::EntityDestructor, Component, ComponentPool, ComponentPoolsTrait, Scene},
    manifestation::Screen,
};
use component::Component;
use engine::engine;

use cgmath::*;

#[derive(Component, Copy, Clone)]
pub struct Transform(pub Matrix4<f32>);

#[derive(Component, Copy, Clone)]
pub struct Position(pub Vector3<f32>);

#[derive(Component, Copy, Clone)]
pub struct Rotation(pub Vector3<f32>);

#[derive(Component, Copy, Clone)]
pub struct Velocity(pub Vector3<f32>);

engine!();
