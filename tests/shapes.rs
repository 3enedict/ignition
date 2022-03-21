use std::time::{Instant, Duration};

extern crate ignition;
use ignition::prelude::*;

const ONE_TRIANGLE: &[Vertex] = &[
    Vertex { position: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

#[any_thread]
#[event_driven]
#[ignore]
#[test]
fn one_triangle() {
    env_logger::init();

    let mut engine = pollster::block_on(Engine::setup_engine());
    let triangle = triangle(&mut engine, &Vec::from(ONE_TRIANGLE), include_wgsl!("shaders/gradient.wgsl"));

    game_loop! (
        draw!(triangle);
    );
}










const TWO_TRIANGLES_ONE_BUFFER: &[Vertex] = &[
    Vertex { position: [ 0.55, -0.5 , 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.55,  0.55, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-0.5 ,  0.55, 0.0], color: [0.0, 0.0, 1.0] },

    Vertex { position: [-0.55,  0.5 , 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.55, -0.55, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5 , -0.55, 0.0], color: [0.0, 0.0, 1.0] },
];

#[any_thread]
#[event_driven]
#[ignore]
#[test]
fn two_triangles_in_one_buffer() {
    let mut engine = pollster::block_on(Engine::setup_engine());
    let triangle = triangle(&mut engine, &Vec::from(TWO_TRIANGLES_ONE_BUFFER), include_wgsl!("shaders/gradient.wgsl"));

    game_loop! (
        draw!(triangle);
    );
}














const TRIANGLE_BUFFER_ONE: &[Vertex] = &[
    Vertex { position: [ 0.55, -0.5 , 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.55,  0.55, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-0.5 ,  0.55, 0.0], color: [0.0, 0.0, 1.0] },
];

const TRIANGLE_BUFFER_TWO: &[Vertex] = &[
    Vertex { position: [-0.55,  0.5 , 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.55, -0.55, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5 , -0.55, 0.0], color: [0.0, 0.0, 1.0] },
];

#[any_thread]
#[ignore]
#[test]
fn alternating_triangles() {
    let mut engine = pollster::block_on(Engine::setup_engine());
    let triangle_one = triangle(&mut engine, &Vec::from(TRIANGLE_BUFFER_ONE), include_wgsl!("shaders/gradient.wgsl"));
    let triangle_two = triangle(&mut engine, &Vec::from(TRIANGLE_BUFFER_TWO), include_wgsl!("shaders/gradient.wgsl"));

    let mut instant = Instant::now();
    let mut swap = true;

    game_loop! (
        if instant.elapsed() > Duration::from_millis(200) { 
            instant = Instant::now();
            swap = !swap;
        }

        if swap {
            draw!(triangle_one);
        } else {
            draw!(triangle_two);
        }
    );
}


















const POLYGON_VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];

const POLYGON_INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

#[any_thread]
#[event_driven]
#[ignore]
#[test]
fn polygon() {
    let mut engine = pollster::block_on(Engine::setup_engine());
    let polygon = indexed_shape(&mut engine, &Vec::from(POLYGON_VERTICES), &Vec::from(POLYGON_INDICES), include_wgsl!("shaders/gradient.wgsl"));

    game_loop! (
        draw!(polygon);
    );
}
