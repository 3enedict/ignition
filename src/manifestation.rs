use wgpu::{Device, Queue};

use crate::Configuration;

pub mod artist;
pub mod lift_off;
pub mod nostalgia;
pub mod race_track;

pub trait Renderer {
    fn new(config: &mut Configuration) -> Self;

    fn device(&self) -> &Device;
    fn queue(&self) -> &Queue;
    fn device_mut(&mut self) -> &mut Device;
    fn queue_mut(&mut self) -> &mut Queue;
}
