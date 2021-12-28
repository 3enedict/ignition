use crate::renderer::VglRenderer;

impl VglRenderer {
    pub fn add_system_setup(mut self, setup: fn(&mut VglRenderer)) -> Self { setup(&mut self); self } 
}
