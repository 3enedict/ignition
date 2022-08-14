extern crate ignition;
use ignition::prelude::*;

#[ignore]
#[test]
fn triangle() {
    let engine = Configuration::default()
        .title("Triangle")
        .any_thread()
        .ignite();

    engine.game_loop(_);
}
