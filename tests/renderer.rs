use std::{thread, time::Duration};


extern crate vgl;

use vgl::VglRenderer;
use vgl::core::parameters::VglRendererParameters;

use vgl::objects::vertex::Vertex;

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




// Square




fn one_square(renderer: &mut VglRenderer) {
    let mut square = vec!
        [
        Vertex { position: [ 0.0,  0.0] },
        ];

    let mut sizes = vec![0.1];

    renderer.add_squares(&mut square, &mut sizes);
}

#[ignore]
#[test]
fn render_one_square() {
    VglRenderer::new(VglRendererParameters::default())
        .add_system_setup(one_square)
        .draw();

    thread::sleep(Duration::from_millis(WAIT_TIME));
}



fn two_squares(renderer: &mut VglRenderer) {
    let mut squares = vec!
        [
        Vertex { position: [ 0.0,  0.0] },

        Vertex { position: [ 0.5,  0.1] },
        ];

    let mut sizes = vec![0.1, 0.3];


    renderer.add_squares(&mut squares, &mut sizes);
}

#[ignore]
#[test]
fn render_two_squares() {
    VglRenderer::new(VglRendererParameters::default())
        .add_system_setup(two_squares)
        .draw();

    thread::sleep(Duration::from_millis(WAIT_TIME));
}




// Shaders





mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 fragColor;

layout(location = 0) in vec2 position;

vec3 colors[3] = vec3[](
vec3(1.0, 0.0, 0.0),
vec3(0.0, 1.0, 0.0),
vec3(0.0, 0.0, 1.0)
   );

   void main() {
   gl_Position = vec4(position, 0.0, 1.0);
   fragColor = colors[gl_VertexIndex % 3];
   }
   "
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
   #version 450
   #extension GL_ARB_separate_shader_objects : enable

   layout(location = 0) in vec3 fragColor;

   layout(location = 0) out vec4 outColor;

   void main() {
   outColor = vec4(fragColor, 1.0);
   }
   "
    }
}

#[ignore]
#[test]
fn render_triangle_with_shaders() {
    VglRenderer::new(VglRendererParameters::default())
        .load_shaders(vs::load, fs::load)
        .add_system_setup(one_triangle)
        .draw();

    thread::sleep(Duration::from_millis(WAIT_TIME));
}
