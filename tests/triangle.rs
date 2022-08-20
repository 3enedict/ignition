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
    let mut engine = Configuration::default().image();

    let vertices: Vec<f32> = vec![
        0.0, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let vertex_buffer = engine.vertex_buffer(vertices);
    let pipeline = engine.pipeline(
        wgpu::include_wgsl!("shaders/gradient.wgsl"),
        engine.renderer.description.format,
    );

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

    {
        let buffer_slice = engine.renderer.buffer.slice(..);

        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        engine.renderer.gpu.device.poll(wgpu::Maintain::Wait);
        pollster::block_on(rx.receive()).unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        use image::{ImageBuffer, Rgba};
        let buffer =
            ImageBuffer::<Rgba<u8>, _>::from_raw(engine.renderer.size, engine.renderer.size, data)
                .unwrap();
        buffer.save("image.png").unwrap();
    }
    engine.renderer.buffer.unmap();
}
