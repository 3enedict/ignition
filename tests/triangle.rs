extern crate ignition;
use ignition::prelude::*;

const ONE_TRIANGLE: &[Vertex] = &[
    Vertex { position: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

#[ignore]
#[test]
fn one_triangle() {
    env_logger::init();

    let mut engine = pollster::block_on(Engine::setup_engine());
    let triangle = Triangle::ignite(&mut engine, &Vec::from(ONE_TRIANGLE), include_wgsl!("shaders/gradient.wgsl"));

    run_return! (
        redraw_requested!( 
            render!(
                render_pass.set_pipeline(&triangle.pipeline);
                render_pass.set_vertex_buffer(0, triangle.vertex_buffer.slice(..));

                render_pass.draw(0..triangle.vertex_len, 0..1);
            );
        );
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

#[ignore]
#[test]
fn two_triangles_in_one_buffer() {
    let mut engine = pollster::block_on(Engine::setup_engine());
    let triangle = Triangle::ignite(&mut engine, &Vec::from(TWO_TRIANGLES_ONE_BUFFER), include_wgsl!("shaders/gradient.wgsl"));

    run_return! (
        redraw_requested!( 
            render!(
                render_pass.set_pipeline(&triangle.pipeline);
                render_pass.set_vertex_buffer(0, triangle.vertex_buffer.slice(..));

                render_pass.draw(0..triangle.vertex_len, 0..1);
            );
        );
    );
}
