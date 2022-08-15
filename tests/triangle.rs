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
    let pipeline = engine.pipeline(wgpu::include_wgsl!("shaders/gradient.wgsl"));

    engine.event_loop(move |engine: &mut Engine| {
        let mut commands = Commands::ignite(engine)?;
        let mut render_pass = commands.ignite_render_pass();

        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);

        drop(render_pass);
        commands.execute(engine)
    });
}
