extern crate ignition;
use ignition::prelude::*;

#[ignore]
#[test]
fn triangle() {
    let engine = Configuration::default()
        .title("Triangle")
        .any_thread()
        .ignite();

    engine.event_loop(move |engine: &mut Engine| {
        let mut commands = Commands::ignite(engine)?;
        let render_pass = commands.ignite_render_pass();

        // Do stuff

        drop(render_pass);
        commands.execute(engine)
    });
}
