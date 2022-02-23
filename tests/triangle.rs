use wgpu::include_wgsl;

extern crate ignition;

use ignition::core::{
    Engine,
    rendering::vertex_buffer::Vertex,
    shapes::ignite_shape,
};

use game_loop::game_loop;

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

#[ignore]
#[test]
fn one_triangle() {
    let mut engine = Engine::ignite();

    ignite_shape(&mut engine, &Vec::from(VERTICES), include_wgsl!("triangle.wgsl"));

    game_loop! (

    );
}
