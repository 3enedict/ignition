extern crate ignition;
use ignition::prelude::*;

#[ignore]
#[test]
fn triangle() {
    let _engine = Configuration::default()
        .title("Triangle")
        .any_thread()
        .ignite();
}
