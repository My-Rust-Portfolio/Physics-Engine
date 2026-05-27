use crate::components::{Circle, Position, Velocity};
use crate::world::World2D;
use core_ecs::Entity;

pub fn spawn_circle(world: &mut World2D, x: f32, y: f32, radius: f32) {
    let entity = world.spawn();

    world.add_position(entity, Position { x, y });
    world.add_velocity(entity, Velocity { dx: 0.0, dy: 0.0 });
    world.add_circle(entity, Circle { radius: radius });
}

pub fn find_circle_at_position(world: &World2D, point: Position) -> Option<Entity> {
    let mut best: Option<(Entity, f32)> = None;

    for (entity, position) in world.positions_iter() {
        if let Some(circle) = world.get_circle(*entity) {
            let dx = point.x - position.x;
            let dy = point.y - position.y;
            let distance_sq = dx * dx + dy * dy;
            let radius_sq = circle.radius * circle.radius;

            if distance_sq <= radius_sq {
                best = Some((*entity, radius_sq));
            }
        }
    }

    best.map(|(entity, _)| entity)
}