extern crate vgl;

use vgl::core::Base;
use base_derive::Base;

use crate::basic_shapes::one_triangle::model::Model;
use crate::basic_shapes::one_triangle::view::View;


#[derive(Base)]
pub struct Controller {
    model: Model,
    view: View,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            model: Model::new(),
            view: View::new(),
        }
    }

    fn setup(&mut self) {
        self.view.setup(self.model.get_object());
    }

    fn update(&mut self) {
        self.view.update()
    }
}

