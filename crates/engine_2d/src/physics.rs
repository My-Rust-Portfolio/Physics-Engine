use crate::components::{Position, Velocity};
use crate::world::World2D;

pub fn step(world: &mut World2D, dt: f32, gravity: f32, ground_y: f32) {
    let mut updates = Vec::new();

    // calculate new velocity with gravity
    for (entity, velocity) in world.velocities_iter() {
        if let Some(position) = world.get_position(*entity) {
            let mut new_velocity = Velocity {
                dx: velocity.dx,
                dy: velocity.dy + gravity * dt,
            };

            let mut new_position = Position {
                x: position.x + new_velocity.dx * dt,
                y: position.y + new_velocity.dy * dt,
            };

            if let Some(circle) = world.get_circle(*entity) {
                let circle_bottom = new_position.y + circle.radius;

                if circle_bottom > ground_y {
                    new_position.y = ground_y - circle.radius;

                    if new_velocity.dy > 0.0 {
                        new_velocity.dy = 0.0;
                    }
                }
            }

            updates.push((*entity, new_position, new_velocity));
        }
    }

    // update positions and velocities with final values
    for (entity, new_position, new_velocity) in updates {
        if let Some(position) = world.get_position_mut(entity) {
            *position = new_position;
        }

        if let Some(velocity) = world.get_velocity_mut(entity) {
            *velocity = new_velocity;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Circle, Position, Velocity};

    #[test]
    fn gravity_increases_downward_velocity() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 0.0, y: 0.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 0.0 });

        step(&mut world, 1.0, 100.0, 1000.0);

        assert_eq!(world.get_velocity(entity), Some(&Velocity { dx: 0.0, dy: 100.0 }));
    }

    #[test]
    fn gravity_moves_object_downward() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 0.0, y: 0.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 0.0 });

        step(&mut world, 1.0, 100.0, 1000.0);

        assert_eq!(world.get_position(entity), Some(&Position { x: 0.0, y: 100.0 }));
    }

    #[test]
    fn circle_stops_on_ground() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 50.0, y: 90.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 50.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 100.0);

        assert_eq!(world.get_position(entity), Some(&Position { x: 50.0, y: 90.0 }));
        assert_eq!(world.get_velocity(entity), Some(&Velocity { dx: 0.0, dy: 0.0 }));
    }

    #[test]
    fn object_falls_again_if_ground_moves_down() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 50.0, y: 90.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 0.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 100.0);
        assert_eq!(world.get_position(entity), Some(&Position { x: 50.0, y: 90.0 }));

        step(&mut world, 1.0, 100.0, 200.0);

        assert_eq!(world.get_velocity(entity), Some(&Velocity { dx: 0.0, dy: 100.0 }));
        assert_eq!(world.get_position(entity), Some(&Position { x: 50.0, y: 190.0 }));
    }
}
