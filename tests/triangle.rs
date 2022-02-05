extern crate vgl;

use vgl::core::rendering::create_renderer;
use vgl::core::rendering::draw;
use vgl::core::objects::VglObject;
use vgl::core::objects::vertex::Vertex;

use game_loop::game_loop;

#[ignore]
#[test]
fn one_triangle() {
    let mut renderer = create_renderer();

    let triangle = VglObject::triangle(&vec!
        [
        Vertex { position: [ 0.0, -0.5] },
        Vertex { position: [ 0.5,  0.5] },
        Vertex { position: [-0.5,  0.5] },
        ]);

    renderer.add_objects(&triangle);

    game_loop! (
        draw(&mut renderer);
    );
}
