extern crate ignition;
use ignition::prelude::*;

#[derive(Component)]
struct RGB {}

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
        let mut commands = engine.encoder()?;
        let mut render_pass = engine.render_pass(&mut commands);

        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);

        drop(render_pass);
        engine.render(commands)
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

    let mut commands = engine.encoder().unwrap();
    let mut render_pass = engine.render_pass(&mut commands);

    render_pass.set_pipeline(&pipeline);
    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
    render_pass.draw(0..3, 0..1);

    drop(render_pass);
    engine.render_to_file(commands, "image.png").unwrap();
}
