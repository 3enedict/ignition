use wgpu::{Adapter, Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

pub mod artist;
pub mod lift_off;
pub mod nostalgia;
pub mod painting;

pub struct Renderer {
    pub screen: Option<Screen>,

    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

pub struct Screen {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub size: PhysicalSize<u32>,

    pub surface: Surface,
    pub config: SurfaceConfiguration,
}
