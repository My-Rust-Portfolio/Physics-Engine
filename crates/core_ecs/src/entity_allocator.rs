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

//////////// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_creates_unique_entities() {
        let mut allocator = EntityAllocator::new();

        let first = allocator.allocate();
        let second = allocator.allocate();

        assert_ne!(first, second);
        assert_ne!(first.get_index(), second.get_index());
        assert!(allocator.is_alive(first));
        assert!(allocator.is_alive(second));
    }

    #[test]
    fn deallocate_marks_entity_as_not_alive() {
        let mut allocator = EntityAllocator::new();

        let entity = allocator.allocate();

        assert!(allocator.is_alive(entity));
        assert!(allocator.deallocate(entity));
        assert!(!allocator.is_alive(entity));
    }

    #[test]
    fn reused_index_gets_new_generation() {
        let mut allocator = EntityAllocator::new();

        let first = allocator.allocate();
        let old_index = first.get_index();
        let old_generation = first.get_generation();

        assert!(allocator.deallocate(first));

        let second = allocator.allocate();

        assert_eq!(second.get_index(), old_index);
        assert_eq!(second.get_generation(), old_generation + 1);
        assert_ne!(first, second);
        assert!(!allocator.is_alive(first));
        assert!(allocator.is_alive(second));
    }

    #[test]
    fn stale_entity_cannot_be_deallocated_twice() {
        let mut allocator = EntityAllocator::new();

        let entity = allocator.allocate();

        assert!(allocator.deallocate(entity));
        assert!(!allocator.deallocate(entity));
    }
}
