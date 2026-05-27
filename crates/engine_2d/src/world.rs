use crate::components::{Circle, Position, Velocity};
use core_ecs::{ComponentStorage, Entity, EntityAllocator};

pub struct World2D {
    allocator: EntityAllocator,
    positions: ComponentStorage<Position>,
    velocities: ComponentStorage<Velocity>,
    circles: ComponentStorage<Circle>,
}

impl World2D {
    pub fn new() -> Self {
        Self {
            allocator: EntityAllocator::new(),
            positions: ComponentStorage::new(),
            velocities: ComponentStorage::new(),
            circles: ComponentStorage::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.allocator.allocate()
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.allocator.deallocate(entity) {
            return false;
        }

        self.positions.remove(entity);
        self.velocities.remove(entity);
        self.circles.remove(entity);
        true
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.allocator.is_alive(entity)
    }

    pub fn add_position(&mut self, entity: Entity, position: Position) -> Option<Position> {
        if !self.is_alive(entity) {
            return None;
        }
        self.positions.insert(entity, position)
    }

    pub fn add_velocity(&mut self, entity: Entity, velocity: Velocity) -> Option<Velocity> {
        if !self.is_alive(entity) {
            return None;
        }
        self.velocities.insert(entity, velocity)
    }

    pub fn add_circle(&mut self, entity: Entity, circle: Circle) -> Option<Circle> {
        if !self.is_alive(entity) {
            return None;
        }

        self.circles.insert(entity, circle)
    }

    pub fn get_position(&self, entity: Entity) -> Option<&Position> {
        if !self.is_alive(entity) {
            return None;
        }

        self.positions.get(entity)
    }

    pub fn get_position_mut(&mut self, entity: Entity) -> Option<&mut Position> {
        if !self.is_alive(entity) {
            return None;
        }

        self.positions.get_mut(entity)
    }

    pub fn get_velocity(&self, entity: Entity) -> Option<&Velocity> {
        if !self.is_alive(entity) {
            return None;
        }

        self.velocities.get(entity)
    }

    pub fn get_velocity_mut(&mut self, entity: Entity) -> Option<&mut Velocity> {
        if !self.is_alive(entity) {
            return None;
        }

        self.velocities.get_mut(entity)
    }

    pub fn get_circle(&self, entity: Entity) -> Option<&Circle> {
        if !self.is_alive(entity) {
            return None;
        }

        self.circles.get(entity)
    }

    pub fn get_circle_mut(&mut self, entity: Entity) -> Option<&mut Circle> {
        if !self.is_alive(entity) {
            return None;
        }

        self.circles.get_mut(entity)
    }

    pub fn positions_iter(&self) -> impl Iterator<Item = (&Entity, &Position)> {
        self.positions.iter()
    }

    pub fn velocities_iter(&self) -> impl Iterator<Item = (&Entity, &Velocity)> {
        self.velocities.iter()
    }

    pub fn circles_iter(&self) -> impl Iterator<Item = (&Entity, &Circle)> {
        self.circles.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Position, Velocity};

    #[test]
    fn spawn_creates_alive_entity() {
        let mut world = World2D::new();

        let entity = world.spawn();

        assert!(world.is_alive(entity));
    }

    #[test]
    fn can_add_and_get_position() {
        let mut world = World2D::new();
        let entity = world.spawn();

        let old = world.add_position(entity, Position { x: 10.0, y: 20.0 });

        assert_eq!(old, None);
        assert_eq!(
            world.get_position(entity),
            Some(&Position { x: 10.0, y: 20.0 })
        );
    }

    #[test]
    fn can_add_and_get_velocity() {
        let mut world = World2D::new();
        let entity = world.spawn();

        let old = world.add_velocity(entity, Velocity { dx: 1.5, dy: -2.0 });

        assert_eq!(old, None);
        assert_eq!(
            world.get_velocity(entity),
            Some(&Velocity { dx: 1.5, dy: -2.0 })
        );
    }

    #[test]
    fn can_add_and_get_circle() {
        let mut world = World2D::new();
        let entity = world.spawn();

        let old = world.add_circle(entity, Circle { radius: 5.0 });

        assert_eq!(old, None);
        assert_eq!(world.get_circle(entity), Some(&Circle { radius: 5.0 }));
    }

    #[test]
    fn can_mutate_position_through_position_mut() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 1.0, y: 2.0 });

        let position = world.get_position_mut(entity).unwrap();
        position.x = 100.0;
        position.y = 200.0;

        assert_eq!(
            world.get_position(entity),
            Some(&Position { x: 100.0, y: 200.0 })
        );
    }

    #[test]
    fn can_mutate_velocity_through_velocity_mut() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_velocity(entity, Velocity { dx: 10.0, dy: 20.0 });

        let velocity = world.get_velocity_mut(entity).unwrap();
        velocity.dx = 100.0;
        velocity.dy = 200.0;

        assert_eq!(
            world.get_velocity(entity),
            Some(&Velocity {
                dx: 100.0,
                dy: 200.0
            })
        );
    }

    #[test]
    fn can_mutate_circle_through_circle_mut() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_circle(entity, Circle { radius: 5.0 });

        let circle = world.get_circle_mut(entity).unwrap();
        circle.radius = 100.0;

        assert_eq!(world.get_circle(entity), Some(&Circle { radius: 100.0 }));
    }

    #[test]
    fn despawn_removes_components_and_kills_entity() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 5.0, y: 6.0 });
        world.add_velocity(entity, Velocity { dx: 7.0, dy: 8.0 });
        world.add_circle(entity, Circle { radius: 5.0 });

        assert!(world.despawn(entity));

        assert!(!world.is_alive(entity));
        assert_eq!(world.get_position(entity), None);
        assert_eq!(world.get_velocity(entity), None);
        assert_eq!(world.get_circle(entity), None);
    }

    #[test]
    fn cannot_add_position_to_dead_entity() {
        let mut world = World2D::new();
        let entity = world.spawn();

        assert!(world.despawn(entity));

        let result = world.add_position(entity, Position { x: 1.0, y: 2.0 });

        assert_eq!(result, None);
        assert_eq!(world.get_position(entity), None);
    }

    #[test]
    fn cannot_add_velocity_to_dead_entity() {
        let mut world = World2D::new();
        let entity = world.spawn();

        assert!(world.despawn(entity));

        let result = world.add_velocity(entity, Velocity { dx: 3.0, dy: 4.0 });

        assert_eq!(result, None);
        assert_eq!(world.get_velocity(entity), None);
    }

    #[test]
    fn cannot_add_circle_to_dead_entity() {
        let mut world = World2D::new();
        let entity = world.spawn();

        assert!(world.despawn(entity));

        let result = world.add_circle(entity, Circle { radius: 5.0 });

        assert_eq!(result, None);
        assert_eq!(world.get_circle(entity), None);
    }

    #[test]
    fn despawning_same_entity_twice_returns_false() {
        let mut world = World2D::new();
        let entity = world.spawn();

        assert!(world.despawn(entity));
        assert!(!world.despawn(entity));
    }

    #[test]
    fn positions_iter_returns_all_entities_with_positions() {
        let mut world = World2D::new();

        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();
        let e4 = world.spawn();
        let e5 = world.spawn();
        let e6 = world.spawn();

        world.add_position(e1, Position { x: 5.0, y: 5.0 });
        world.add_position(e2, Position { x: 10.0, y: 10.0 });
        world.add_velocity(e3, Velocity { dx: 1.0, dy: 1.0 });
        world.add_velocity(e4, Velocity { dx: 20.0, dy: 20.0 });
        world.add_circle(e5, Circle { radius: 5.0 });
        world.add_circle(e6, Circle { radius: 15.0 });

        let mut count = 0;
        for (entity, _pos) in world.positions_iter() {
            count += 1;
            assert!(*entity == e1 || *entity == e2);
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn velocities_iter_returns_all_entities_with_velocities() {
        let mut world = World2D::new();

        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();
        let e4 = world.spawn();
        let e5 = world.spawn();
        let e6 = world.spawn();

        world.add_position(e1, Position { x: 5.0, y: 5.0 });
        world.add_position(e2, Position { x: 10.0, y: 10.0 });
        world.add_velocity(e3, Velocity { dx: 1.0, dy: 1.0 });
        world.add_velocity(e4, Velocity { dx: 20.0, dy: 20.0 });
        world.add_circle(e5, Circle { radius: 5.0 });
        world.add_circle(e6, Circle { radius: 15.0 });

        let mut count = 0;
        for (entity, _pos) in world.velocities_iter() {
            count += 1;
            assert!(*entity == e3 || *entity == e4);
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn circles_iter_returns_all_entities_with_circles() {
        let mut world = World2D::new();

        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();
        let e4 = world.spawn();
        let e5 = world.spawn();
        let e6 = world.spawn();

        world.add_position(e1, Position { x: 5.0, y: 5.0 });
        world.add_position(e2, Position { x: 10.0, y: 10.0 });
        world.add_velocity(e3, Velocity { dx: 1.0, dy: 1.0 });
        world.add_velocity(e4, Velocity { dx: 20.0, dy: 20.0 });
        world.add_circle(e5, Circle { radius: 5.0 });
        world.add_circle(e6, Circle { radius: 15.0 });

        let mut count = 0;
        for (entity, _pos) in world.circles_iter() {
            count += 1;
            assert!(*entity == e5 || *entity == e6);
        }

        assert_eq!(count, 2);
    }
}
