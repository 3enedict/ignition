use wgpu::include_wgsl;

extern crate ignition;

use ignition::core::Engine;

use game_loop::game_loop;

#[ignore]
#[test]
fn one_triangle() {
    let mut engine = Engine::ignite();

    engine.add_pipeline(&include_wgsl!("triangle.wgsl"));

    game_loop! (

    );
}
