use crate::ecs::IgnitionScene;
use bit_set::BitSet;

pub struct Entity {
    pub id: usize,
    pub bitmask: BitSet,
}

impl IgnitionScene {
    pub fn entity(&mut self) -> Entity {
        let new_entity = Entity {
            id: self.available_entities.pop().unwrap(),
            bitmask: BitSet::new(),
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

    pub fn delete_entity(&mut self, entity: Entity) {
        self.available_entities.push(entity.id);
    }
}
