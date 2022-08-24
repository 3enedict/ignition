use wgpu::{Buffer, Device, Queue, Texture, TextureDescriptor, TextureView};

use crate::{
    liberty::Configuration,
    manifestation::{lift_off::headless::Headless, Renderer},
};

pub struct Image<'a> {
    pub texture: Texture,
    pub description: TextureDescriptor<'a>,
    pub size: u32,
    pub view: TextureView,
    pub buffer: Buffer,

    pub gpu: Headless,
}

impl Renderer for Image<'_> {
    fn new(config: &mut Configuration) -> Self {
        let gpu = Headless::new(config);

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
