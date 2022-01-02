use std::{thread, time::Duration};


extern crate vgl;

use vgl::renderer::VglRenderer;
use vgl::renderer::core::parameters::VglRendererParameters;

use vgl::object::vertex::Vertex;

const WAIT_TIME: u64 = 500;




// Triangles




fn one_triangle(renderer: &mut VglRenderer) {
    let mut triangle = vec!
        [
            Vertex { position: [ 0.0, -0.5] },
            Vertex { position: [ 0.5,  0.5] },
            Vertex { position: [-0.5,  0.5] },
        ];

    renderer.add_triangles(&mut triangle);
}

#[ignore]
#[test]
fn render_one_triangle() {
    VglRenderer::new(VglRendererParameters::default())
        .add_system_setup(one_triangle)
        .draw();

    thread::sleep(Duration::from_millis(WAIT_TIME));
}



fn two_triangles(renderer: &mut VglRenderer) {
    let mut triangles = vec!
        [
            Vertex { position: [ 0.55, -0.5 ] },
            Vertex { position: [ 0.55,  0.55] },
            Vertex { position: [-0.5 ,  0.55] },

            Vertex { position: [-0.55,  0.5 ] },
            Vertex { position: [-0.55, -0.55] },
            Vertex { position: [ 0.5 , -0.55] },
        ];

    renderer.add_triangles(&mut triangles);
}

#[ignore]
#[test]
fn render_two_triangles() {
    VglRenderer::new(VglRendererParameters::default())
        .add_system_setup(two_triangles)
        .draw();

    thread::sleep(Duration::from_millis(WAIT_TIME));
}




// Rectangle




fn one_rectangle(renderer: &mut VglRenderer) {
    let mut rectangle = vec!
        [
            Vertex{ position: [-0.5,  0.5] },
            Vertex{ position: [ 0.5, -0.5] },
        ];

    renderer.add_rectangles(&mut rectangle);
}

#[ignore]
#[test]
fn render_one_rectangle() {
    VglRenderer::new(VglRendererParameters::default())
        .add_system_setup(one_rectangle)
        .draw();

    thread::sleep(Duration::from_millis(WAIT_TIME));
}



fn two_rectangles(renderer: &mut VglRenderer) {
    let mut rectangle = vec!
        [
            Vertex{ position: [-0.25,  0.75] },
            Vertex{ position: [-0.75, -0.75] },

            Vertex{ position: [ 0.75,  0.75] },
            Vertex{ position: [ 0.25, -0.75] },
        ];

    renderer.add_rectangles(&mut rectangle);
}

#[ignore]
#[test]
fn render_two_rectangles() {
    VglRenderer::new(VglRendererParameters::default())
        .add_system_setup(two_rectangles)
        .draw();

    thread::sleep(Duration::from_millis(WAIT_TIME));
}
