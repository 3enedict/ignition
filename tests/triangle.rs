use std::num::NonZeroU32;

extern crate ignition;
use ignition::prelude::*;

#[ignore]
#[test]
fn triangle() {
    let mut engine = Configuration::default()
        .title("Triangle")
        .any_thread()
        .ignite();

    let vertices: Vec<f32> = vec![
        0.0, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let vertex_buffer = engine.vertex_buffer(vertices);
    let pipeline = engine.pipeline(
        wgpu::include_wgsl!("shaders/gradient.wgsl"),
        engine.renderer.config.format,
    );

    engine.event_loop(move |engine: &mut Engine<Screen>| {
        let mut commands = Commands::ignite(engine)?;
        let mut render_pass = commands.ignite_render_pass();

        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);

        drop(render_pass);
        commands.execute(engine)
    });
}

#[ignore]
#[test]
fn headless() {
    pollster::block_on(run());
}

async fn run() {
    let mut engine = Configuration::default().headless();

    let texture_size = 256u32;
    let texture_desc = wgpu::TextureDescriptor {
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
    let (texture, texture_view) = engine.texture(&texture_desc);

    let u32_size = std::mem::size_of::<u32>() as u32;
    let output_buffer_size = (u32_size * texture_size * texture_size) as wgpu::BufferAddress;
    let output_buffer_desc = wgpu::BufferDescriptor {
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };
    let output_buffer = engine.buffer(output_buffer_desc);

    let vertices: Vec<f32> = vec![
        0.0, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let vertex_buffer = engine.vertex_buffer(vertices);
    let pipeline = engine.pipeline(
        wgpu::include_wgsl!("shaders/gradient.wgsl"),
        texture_desc.format,
    );

    let mut encoder = engine
        .renderer
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let render_pass_desc = wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: true,
            },
        })],
        depth_stencil_attachment: None,
    };
    let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

    render_pass.set_pipeline(&pipeline);
    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
    render_pass.draw(0..3, 0..1);

    drop(render_pass);

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(u32_size * texture_size),
                rows_per_image: NonZeroU32::new(texture_size),
            },
        },
        texture_desc.size,
    );

    engine.renderer.queue.submit(Some(encoder.finish()));

    {
        let buffer_slice = output_buffer.slice(..);

        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        engine.renderer.device.poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        use image::{ImageBuffer, Rgba};
        let buffer =
            ImageBuffer::<Rgba<u8>, _>::from_raw(texture_size, texture_size, data).unwrap();
        buffer.save("image.png").unwrap();
    }
    output_buffer.unmap();
}
