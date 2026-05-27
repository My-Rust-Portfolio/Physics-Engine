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

    pub fn iter(&self) -> impl Iterator<Item = (&Entity, &T)> {
    self.components.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Entity, &mut T)> {
        self.components.iter_mut()
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

    #[test]
    fn iter_returns_all_components() {
        let mut storage = ComponentStorage::new();
        let e1 = Entity::new(0, 0);
        let e2 = Entity::new(1, 0);

        storage.insert(e1, Position { x: 1.0, y: 2.0 });
        storage.insert(e2, Position { x: 3.0, y: 4.0 });

        let mut count = 0;
        for (entity, pos) in storage.iter() {
            count += 1;
            assert!(entity.get_index() == 0 || entity.get_index() == 1);
            assert!(pos.x == 1.0 || pos.x == 3.0);
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn iter_mut_allows_mutation_of_all_components() {
        let mut storage = ComponentStorage::new();
        let e1 = Entity::new(0, 0);
        let e2 = Entity::new(1, 0);

        storage.insert(e1, Position { x: 1.0, y: 2.0 });
        storage.insert(e2, Position { x: 3.0, y: 4.0 });

        for (_entity, pos) in storage.iter_mut() {
            pos.x += 10.0;
        }

        assert_eq!(storage.get(e1), Some(&Position { x: 11.0, y: 2.0 }));
        assert_eq!(storage.get(e2), Some(&Position { x: 13.0, y: 4.0 }));
    }
}
