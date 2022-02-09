extern crate vgl;

use vgl::core::rendering::create_renderer;

use game_loop::game_loop;

#[ignore]
#[test]
fn one_triangle() {
    let mut renderer = create_renderer();

    game_loop! (

    );
}
