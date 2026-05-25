use crate::Entity;
use std::collections::HashMap;

pub struct ComponentStorage<T> {
    components: HashMap<Entity, T>,
}

impl<T> ComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T) -> Option<T> {
        self.components.insert(entity, component)
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.components.get(&entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(&entity)
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        self.components.remove(&entity)
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.components.contains_key(&entity)
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Entity;

    // example component
    #[derive(Debug, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[test]
    fn new_storage_is_empty() {
        let storage = <ComponentStorage<Position>>::new();
        assert!(storage.is_empty());
        assert_eq!(storage.len(), 0);
    }

    #[test]
    fn insert_and_get_component() {
        let mut storage = ComponentStorage::new();
        let entity = Entity::new(0, 0);

        storage.insert(entity, Position { x: 1.0, y: 2.0 });

        let position = storage.get(entity);
        assert_eq!(position, Some(&Position { x: 1.0, y: 2.0 }));
    }

    #[test]
    fn insert_replaces_existing_component() {
        let mut storage = ComponentStorage::new();
        let entity = Entity::new(0, 0);

        let old = storage.insert(entity, Position { x: 1.0, y: 2.0 });
        assert_eq!(old, None);

        let old = storage.insert(entity, Position { x: 5.0, y: 6.0 });
        assert_eq!(old, Some(Position { x: 1.0, y: 2.0 }));

        assert_eq!(storage.get(entity), Some(&Position { x: 5.0, y: 6.0 }));
    }

    #[test]
    fn get_mut_allows_modification() {
        let mut storage = ComponentStorage::new();
        let entity = Entity::new(0, 0);

        storage.insert(entity, Position { x: 1.0, y: 2.0 });

        let position = storage.get_mut(entity).unwrap();
        position.x = 10.0;
        position.y = 20.0;

        assert_eq!(storage.get(entity), Some(&Position { x: 10.0, y: 20.0 }));
    }

    #[test]
    fn remove_deletes_component() {
        let mut storage = ComponentStorage::new();
        let entity = Entity::new(0, 0);

        storage.insert(entity, Position { x: 1.0, y: 2.0 });

        let removed = storage.remove(entity);
        assert_eq!(removed, Some(Position { x: 1.0, y: 2.0 }));
        assert_eq!(storage.get(entity), None);
        assert!(!storage.contains(entity));
    }
}
