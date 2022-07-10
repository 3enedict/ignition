use std::time::{Duration, Instant};

extern crate ignition;
use ignition::{prelude::*, renderer::shapes::Renderable};

#[ignore]
#[test]
fn alternating_triangles() {
    let mut engine = Engine::ignite();

    let triangle1 = engine
        .xy([0.55, -0.5, 0.55, 0.55, -0.5, 0.55])
        .rgb([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
        .with_component(include_wgsl!("shaders/gradient.wgsl"))
        .doritos();

    let triangle2 = engine
        .xyz([-0.55, 0.5, 0.0, -0.55, -0.55, 0.0, 0.5, -0.55, 0.0])
        .rgb([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
        .with_component(include_wgsl!("shaders/gradient.wgsl"))
        .doritos();

    engine.scene.disable::<Box<dyn Renderable>>(triangle1);

    let mut instant = Instant::now();

    engine.game_loop(move |engine: &mut Engine| {
        if instant.elapsed() > Duration::from_millis(200) {
            instant = Instant::now();

            engine.scene.toggle::<Box<dyn Renderable>>(triangle1);
            engine.scene.toggle::<Box<dyn Renderable>>(triangle2);
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
