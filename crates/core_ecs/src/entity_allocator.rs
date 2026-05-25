use crate::Entity;

pub struct EntityAllocator {
    generations: Vec<u32>,
    free_indices: Vec<u32>,
}

impl EntityAllocator {
    pub fn new() -> Self {
        Self {
            generations: Vec::new(),
            free_indices: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> Entity {
        if let Some(index) = self.free_indices.pop() {
            Entity::new(index, self.generations[index as usize])
        } else {
            let index = self.generations.len() as u32;
            self.generations.push(0);
            Entity::new(index, 0)
        }
    }

    pub fn deallocate(&mut self, entity: Entity) -> bool {
        let index = entity.get_index() as usize;

        if index < self.generations.len() && self.generations[index] == entity.get_generation() {
            self.generations[index] += 1;
            self.free_indices.push(entity.get_index());
            true
        } else {
            false
        }
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        let index = entity.get_index() as usize;
        index < self.generations.len() && self.generations[index] == entity.get_generation()
    }
}
