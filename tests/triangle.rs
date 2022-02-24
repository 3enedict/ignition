use wgpu::include_wgsl;

extern crate ignition;

use ignition::core::{
    Engine,
    rendering::vertex_buffer::Vertex,
    shapes::ignite_shape,
    options::IgnitionOptions,
};

mod utils;
use crate::utils::run_game_loop;


const ONE_TRIANGLE: &[Vertex] = &[
    Vertex { position: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

#[ignore]
#[test]
fn one_triangle() {
    env_logger::init();

    let mut engine = pollster::block_on(Engine::setup_engine(
            IgnitionOptions {
                ..Default::default()
            }
    ));

    ignite_shape(&mut engine, &Vec::from(ONE_TRIANGLE), include_wgsl!("shaders/gradient.wgsl"));

    run_game_loop(&mut engine);
}

const TWO_TRIANGLES_ONE_BUFFER: &[Vertex] = &[
    Vertex { position: [ 0.55, -0.5 , 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.55,  0.55, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-0.5 ,  0.55, 0.0], color: [0.0, 0.0, 1.0] },

    Vertex { position: [-0.55,  0.5 , 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.55, -0.55, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5 , -0.55, 0.0], color: [0.0, 0.0, 1.0] },
];

#[ignore]
#[test]
fn two_triangles_in_one_buffer() {
    let mut engine = pollster::block_on(Engine::setup_engine(
            IgnitionOptions {
                ..Default::default()
            }
    ));

    ignite_shape(&mut engine, &Vec::from(TWO_TRIANGLES_ONE_BUFFER), include_wgsl!("shaders/gradient.wgsl"));

    run_game_loop(&mut engine);
}
