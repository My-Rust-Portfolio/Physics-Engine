use crate::components::{Position, Velocity};
use crate::world::World2D;
use core_ecs::Entity;

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

    handle_circle_circle_collisions(world);
}

fn handle_circle_circle_collisions(world: &mut World2D) {
    let circles: Vec<(Entity, Position, f32)> = world
        .positions_iter()
        .filter_map(|(entity, position)| {
            world
                .get_circle(*entity)
                .map(|circle| (*entity, *position, circle.radius))
        })
        .collect();

    let mut position_corrections: Vec<(Entity, f32, f32)> = Vec::new();

    for i in 0..circles.len() {
        for j in (i + 1)..circles.len() {
            let (entity_a, position_a, radius_a) = circles[i];
            let (entity_b, position_b, radius_b) = circles[j];

            let dx = position_b.x - position_a.x;
            let dy = position_b.y - position_a.y;
            let distance_sq = dx * dx + dy * dy;

            let min_distance = radius_a + radius_b;
            let min_distance_sq = min_distance * min_distance;

            if distance_sq < min_distance_sq {
                let distance = distance_sq.sqrt();

                let (normal_x, normal_y) = if distance > 0.0001 {
                    (dx / distance, dy / distance)
                } else {
                    (1.0, 0.0)
                };

                let overlap = min_distance - distance;
                let correction_x = normal_x * overlap * 0.5;
                let correction_y = normal_y * overlap * 0.5;

                position_corrections.push((entity_a, -correction_x, -correction_y));
                position_corrections.push((entity_b, correction_x, correction_y));
            }
        }
    }

    for (entity, correction_x, correction_y) in position_corrections {
        if let Some(position) = world.get_position_mut(entity) {
            position.x += correction_x;
            position.y += correction_y;
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

#[test]
fn overlapping_circles_are_separated() {
    let mut world = World2D::new();

    let a = world.spawn();
    world.add_position(a, Position { x: 100.0, y: 100.0 });
    world.add_velocity(a, Velocity { dx: 0.0, dy: 0.0 });
    world.add_circle(a, Circle { radius: 20.0 });

    let b = world.spawn();
    world.add_position(b, Position { x: 110.0, y: 100.0 });
    world.add_velocity(b, Velocity { dx: 0.0, dy: 0.0 });
    world.add_circle(b, Circle { radius: 20.0 });

    step(&mut world, 0.0, 0.0, 1000.0);

    let pos_a = world.position(a).unwrap();
    let pos_b = world.position(b).unwrap();

    let dx = pos_b.x - pos_a.x;
    let dy = pos_b.y - pos_a.y;
    let distance = (dx * dx + dy * dy).sqrt();

    assert!(distance >= 40.0 - 0.001);
}

#[test]
fn non_overlapping_circles_are_unchanged() {
    let mut world = World2D::new();

    let a = world.spawn();
    world.add_position(a, Position { x: 100.0, y: 100.0 });
    world.add_velocity(a, Velocity { dx: 0.0, dy: 0.0 });
    world.add_circle(a, Circle { radius: 20.0 });

    let b = world.spawn();
    world.add_position(b, Position { x: 200.0, y: 100.0 });
    world.add_velocity(b, Velocity { dx: 0.0, dy: 0.0 });
    world.add_circle(b, Circle { radius: 20.0 });

    step(&mut world, 0.0, 0.0, 1000.0);

    assert_eq!(world.position(a), Some(&Position { x: 100.0, y: 100.0 }));
    assert_eq!(world.position(b), Some(&Position { x: 200.0, y: 100.0 }));
}