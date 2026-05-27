use crate::components::{Circle, Position, Velocity};
use crate::world::World2D;

pub fn spawn_circle(world: &mut World2D, x: f32, y: f32, radius: f32) {
    let entity = world.spawn();

    world.add_position(entity, Position { x, y });
    world.add_velocity(entity, Velocity { dx: 0.0, dy: 0.0 });
    world.add_circle(entity, Circle { radius: radius });
}
