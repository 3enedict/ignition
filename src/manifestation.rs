use log::info;

use wgpu::{
    Buffer, Device, Instance, Queue, Surface, SurfaceConfiguration, Texture, TextureDescriptor,
    TextureView,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

use crate::{
    manifestation::lift_off::{
        create_surface, create_window, generate_default_configuration, get_adapter, get_device,
        get_headless_device,
    },
    Configuration,
};

pub mod artist;
pub mod lift_off;
pub mod nostalgia;
pub mod painting;

pub trait Renderer {
    fn new(config: &Configuration) -> Self;

    fn device(&self) -> &Device;
    fn queue(&self) -> &Queue;
    fn device_mut(&mut self) -> &mut Device;
    fn queue_mut(&mut self) -> &mut Queue;
}

pub struct GPU {
    pub device: Device,
    pub queue: Queue,
}

impl Renderer for GPU {
    fn new(config: &Configuration) -> Self {
        let instance = Instance::new(config.backend);
        let adapter = pollster::block_on(get_adapter(&instance, None));
        let (device, queue) = pollster::block_on(get_headless_device(&adapter));

        info!("Device name : {}", adapter.get_info().name);

        Self { device, queue }
    }

    fn device(&self) -> &Device {
        &self.device
    }

    fn queue(&self) -> &Queue {
        &self.queue
    }

    fn device_mut(&mut self) -> &mut Device {
        &mut self.device
    }

    fn queue_mut(&mut self) -> &mut Queue {
        &mut self.queue
    }
}

pub struct Screen {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub size: PhysicalSize<u32>,
    pub surface: Surface,
    pub config: SurfaceConfiguration,

    pub gpu: GPU,
}

impl Renderer for Screen {
    fn new(config: &Configuration) -> Self {
        let instance = Instance::new(config.backend);
        let (event_loop, window, size) = create_window(config);
        let surface = create_surface(&instance, &window);

        let adapter = pollster::block_on(get_adapter(&instance, Some(&surface)));
        let (device, queue) = pollster::block_on(get_device(&adapter));

        let config = generate_default_configuration(&size, &surface, &adapter);
        surface.configure(&device, &config);

        info!("Device name : {}", adapter.get_info().name);

        Self {
            event_loop: Some(event_loop),
            window,
            size,
            surface,
            config,

            gpu: GPU { device, queue },
        }
    }

    fn device(&self) -> &Device {
        &self.gpu.device
    }

    fn queue(&self) -> &Queue {
        &self.gpu.queue
    }

    fn device_mut(&mut self) -> &mut Device {
        &mut self.gpu.device
    }

    fn queue_mut(&mut self) -> &mut Queue {
        &mut self.gpu.queue
    }
}

pub struct Image<'a> {
    pub texture: Texture,
    pub description: TextureDescriptor<'a>,
    pub size: u32,
    pub view: TextureView,
    pub buffer: Buffer,

    pub gpu: GPU,
}

impl Renderer for Image<'_> {
    fn new(config: &Configuration) -> Self {
        let gpu = GPU::new(config);

        let texture_size = 256u32;
        let description = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: texture_size,
                height: texture_size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        };
        let texture = gpu.device().create_texture(&description);
        let view = texture.create_view(&Default::default());

        let u32_size = std::mem::size_of::<u32>() as u32;
        let buffer_size = (u32_size * texture_size * texture_size) as wgpu::BufferAddress;
        let buffer_desc = wgpu::BufferDescriptor {
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };
        let buffer = gpu.device().create_buffer(&buffer_desc);

        Self {
            texture,
            description,
            size: texture_size,
            view,
            buffer,

            gpu,
        }
    }

    fn device(&self) -> &Device {
        &self.gpu.device
    }

    fn queue(&self) -> &Queue {
        &self.gpu.queue
    }

    fn device_mut(&mut self) -> &mut Device {
        &mut self.gpu.device
    }

    fn queue_mut(&mut self) -> &mut Queue {
        &mut self.gpu.queue
    }
}
