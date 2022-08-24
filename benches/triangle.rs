use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ignition::prelude::*;
use std::num::NonZeroU32;

pub fn record_command_buffer(c: &mut Criterion) {
    let mut engine = Configuration::default().image();

    let vertices: Vec<f32> = vec![
        0.0, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let vertex_buffer = engine.vertex_buffer(vertices);
    let pipeline = engine.pipeline(
        wgpu::include_wgsl!("shaders/gradient.wgsl"),
        engine.renderer.description.format,
    );

    let image = image::io::Reader::open("image.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_bytes();

    c.bench_function("Record command buffer", |b| {
        b.iter(|| {
            let mut encoder = engine
                .renderer
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &engine.renderer.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
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
                    texture: &engine.renderer.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                wgpu::ImageCopyBuffer {
                    buffer: &engine.renderer.buffer,
                    layout: wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: NonZeroU32::new(
                            std::mem::size_of::<u32>() as u32 * engine.renderer.size,
                        ),
                        rows_per_image: NonZeroU32::new(engine.renderer.size),
                    },
                },
                engine.renderer.description.size,
            );

            engine.renderer.queue().submit(Some(encoder.finish()));

            pollster::block_on(compare(&mut engine, &image));

            engine.renderer.buffer.unmap();
        })
    });
}

async fn compare(engine: &mut Engine<Image<'_>>, image: &Vec<u8>) {
    let buffer_slice = engine.renderer.buffer.slice(..);

    let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
    });
    engine.renderer.device().poll(wgpu::Maintain::Wait);
    rx.receive().await.unwrap().unwrap();

    let data = buffer_slice.get_mapped_range();

    if data.to_vec() != *image {
        panic!("Error: Outputed image from GPU doesn't correspond to reference image");
    }
}

pub fn triangle_creation(c: &mut Criterion) {
    let mut engine = Configuration::default().headless();

    c.bench_function("Create simple triangle", |b| {
        b.iter(|| {
            let vertices: Vec<f32> = vec![
                0.0, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0,
                0.0, 1.0,
            ];

            engine.vertex_buffer(black_box(vertices));
            engine.pipeline(
                black_box(wgpu::include_wgsl!("shaders/gradient.wgsl")),
                black_box(wgpu::TextureFormat::Rgba8UnormSrgb),
            );
        })
    });
}

criterion_group!(benches, record_command_buffer, triangle_creation);
criterion_main!(benches);
