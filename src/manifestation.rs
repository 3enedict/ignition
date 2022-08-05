use wgpu::{Adapter, Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

pub mod lift_off;

pub struct Renderer {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub size: PhysicalSize<u32>,

    pub surface: Surface,
    pub config: SurfaceConfiguration,

    pub adapter: Adapter,

    pub device: Device,
    pub queue: Queue,
}
