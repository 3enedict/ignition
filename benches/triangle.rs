use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ignition::prelude::*;

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

criterion_group!(benches, triangle_creation);
criterion_main!(benches);
