mod model;
mod view;
mod controller;

use controller::Controller;
use vgl::core::Base;

#[ignore]
#[test]
fn one_triangle() {
    Controller::new()
        .run();
}
