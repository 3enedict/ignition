use crate::ecs::IgnitionScene;

pub struct Entity {
    pub id: usize,
}

impl IgnitionScene {
    pub fn entity(&mut self) -> Entity {
        let new_entity = Entity {
            id: self.available_entities.pop().unwrap(),
        };

        self.entity_count += 1;
        if self.available_entities.is_empty() {
            for component_pool in self.component_pools.iter_mut() {
                component_pool.push_none();
            }

            self.available_entities.push(self.entity_count);
        }

        new_entity
    }

    pub fn delete(&mut self, entity: Entity) {
        self.entity_count -= 1;
        self.available_entities.push(entity.id);

        for component_pool in self.component_pools.iter_mut() {
            component_pool.delete_entity(entity.id);
        }
    }
}
