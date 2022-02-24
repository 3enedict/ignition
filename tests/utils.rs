use std::time::{Instant, Duration};

extern crate ignition;

use ignition::core::Engine;

use game_loop_that_does_not_hijack_main_thread::game_loop_that_does_not_hijack_main_thread;

const WAIT_TIME: u64 = 500;


pub fn run_game_loop(engine: &mut Engine) {
    let instant = Instant::now();

    game_loop_that_does_not_hijack_main_thread! (
        let elapsed = instant.elapsed();

        println!("{:?}", elapsed);
        if elapsed > Duration::from_millis(WAIT_TIME) {
            *control_flow = ControlFlow::Exit;
        }
    );
}
