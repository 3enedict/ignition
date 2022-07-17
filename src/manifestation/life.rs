use crate::{manifestation::apex::VertexGroup, Engine};

impl Engine {
    pub fn component<G: 'static>(&mut self, component: G) -> &mut Self {
        let entity = self.scene.get_current_entity();
        self.scene.component(entity, component);

        self
    }

    pub fn entity(&mut self) -> usize {
        self.scene.entity()
    }

    pub fn doritos(&mut self) -> usize {
        return self.scene.entity();
    }
}

#[cfg(test)]
mod tests {
    use crate::Engine;
}
