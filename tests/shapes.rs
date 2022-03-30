use std::time::{Instant, Duration};

extern crate ignition;
use ignition::prelude::*;

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
/*
#[any_thread]
#[ignore]
#[test]
fn alternating_triangles() {
env_logger::init();

let mut engine = pollster::block_on(Engine::setup_engine());
let triangle_one = doritos(&mut engine, &Vec::from(TRIANGLE_BUFFER_ONE), include_wgsl!("shaders/gradient.wgsl"));
let triangle_two = doritos(&mut engine, &Vec::from(TRIANGLE_BUFFER_TWO), include_wgsl!("shaders/gradient.wgsl"));

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
Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] },
Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] },
Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] },
Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] },
Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] },
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



*/







const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.8, -0.8, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.2, -0.8, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.2,  0.8, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.8,  0.8, 0.0], color: [0.5, 0.0, 0.5] },
];
const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

#[any_thread]
#[event_driven]
#[ignore]
#[test]
fn rectangle() {
    let mut engine = pollster::block_on(Engine::setup_engine());
    let rectangles = indexed_shape(&mut engine, &Vec::from(VERTICES), &Vec::from(INDICES), include_wgsl!("shaders/gradient.wgsl"));

    game_loop! (
        draw!(rectangles);
    );
}




const RECTANGLES: &[Vertex] = &[
    Vertex { position: [-0.8,  0.8, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.2, -0.8, 0.0], color: [0.5, 0.0, 0.5] },

    /*
       Vertex { position: [ 0.2,  0.8, 0.0], color: [1.0, 0.0, 0.0] },
       Vertex { position: [ 0.8, -0.8, 0.0], color: [0.0, 1.0, 0.0] },
       */
];

#[any_thread]
#[event_driven]
#[ignore]
#[test]
fn rectangles() {
    let mut engine = pollster::block_on(Engine::setup_engine());
    let rectangles = crackers(&mut engine, &Vec::from(RECTANGLES), include_wgsl!("shaders/gradient.wgsl"));

    game_loop! (
        draw!(rectangles);
    );
}
