use crate::ecs::IgnitionScene;

/* This is only really for aesthetics */

pub struct ComponentBuilder<'a> {
    pub scene: &'a mut IgnitionScene,
    pub entity: usize,
}

impl<'a> ComponentBuilder<'a> {
    pub fn with_component<G: 'static>(self, component: G) -> Self {
        self.scene.component(self.entity, component);

        self
    }

    pub fn entity(self) -> usize {
        self.entity
    }
}
