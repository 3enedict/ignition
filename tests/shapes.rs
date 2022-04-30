use std::time::{Duration, Instant};

extern crate ignition;
use ignition::prelude::*;

pub struct Vertices {
    pub vertices: Vec<Vertex>,
}

const TRIANGLE_BUFFER_ONE: &[Vertex] = &[
    Vertex {
        position: [0.55, -0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.55, 0.55, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.55, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

const TRIANGLE_BUFFER_TWO: &[Vertex] = &[
    Vertex {
        position: [-0.55, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.55, -0.55, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.55, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

#[ignore]
#[test]
fn alternating_triangles() {
    let mut engine = Engine::ignite();

    let triangle_one = doritos(
        &mut engine,
        &Vec::from(TRIANGLE_BUFFER_ONE),
        include_wgsl!("shaders/gradient.wgsl"),
    );

    let triangle_two = doritos(
        &mut engine,
        &Vec::from(TRIANGLE_BUFFER_TWO),
        include_wgsl!("shaders/gradient.wgsl"),
    );

    let mut entity_one = engine.scene.entity();
    engine.scene.component(
        &mut entity_one,
        Vertices {
            vertices: Vec::from(TRIANGLE_BUFFER_ONE),
        },
    );
    engine.scene.component(&mut entity_one, triangle_one);
    engine.scene.component(&mut entity_one, true);

    let mut entity_two = engine.scene.entity();
    engine.scene.component(
        &mut entity_two,
        Vertices {
            vertices: Vec::from(TRIANGLE_BUFFER_TWO),
        },
    );
    engine.scene.component(&mut entity_two, triangle_two);
    engine.scene.component(&mut entity_two, true);

    let mut instant = Instant::now();
    let mut swap = true;

    engine.game_loop(move |engine: &mut Engine| {
        if instant.elapsed() > Duration::from_millis(200) {
            instant = Instant::now();
            swap = !swap;
        }

        if swap {
            engine.scene.get_components(2)[0] = Some(true);
            engine.scene.get_components(2)[1] = Some(false);
        } else {
            engine.scene.get_components(2)[1] = Some(true);
            engine.scene.get_components(2)[0] = Some(false);
        }
    });
}

/*
const POLYGON_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    },
];

const POLYGON_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

#[ignore]
#[test]
fn polygon() {
    let options = OptionsBuilder::default().any_thread(true).build();
    let mut engine = Engine::ignite(options);

    let polygon = indexed_shape(
        &mut engine,
        &Vec::from(POLYGON_VERTICES),
        &Vec::from(POLYGON_INDICES),
        include_wgsl!("shaders/gradient.wgsl"),
    );

    game_loop! (
        draw!(polygon);
    );
}
*/
