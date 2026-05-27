use crate::components::{Position, Velocity};
use crate::world::World2D;
use core_ecs::Entity;

pub fn step(
    world: &mut World2D,
    dt: f32,
    gravity: f32,
    viewport_width: f32,
    viewport_height: f32,
    ground_y: f32,
    bouncing_factor: f32,
) {
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
                let radius = circle.radius;

                if new_position.x - radius < 0.0 {
                    new_position.x = radius;

                    if new_velocity.dx < 0.0 {
                        new_velocity.dx = -new_velocity.dx * bouncing_factor;
                    }
                }

                if new_position.x + radius > viewport_width {
                    new_position.x = viewport_width - radius;

                    if new_velocity.dx > 0.0 {
                        new_velocity.dx = -new_velocity.dx * bouncing_factor;
                    }
                }

                if new_position.y - radius < 0.0 {
                    new_position.y = radius;

                    if new_velocity.dy < 0.0 {
                        new_velocity.dy = -new_velocity.dy * bouncing_factor;
                    }
                }

                let floor_y = ground_y.min(viewport_height);
                let circle_bottom = new_position.y + radius;

                if circle_bottom > floor_y {
                    new_position.y = floor_y - radius;

                    if new_velocity.dy > 0.0 {
                        new_velocity.dy = -new_velocity.dy * bouncing_factor;
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

    handle_circle_circle_collisions(world, bouncing_factor);
}

fn handle_circle_circle_collisions(world: &mut World2D, bouncing_factor: f32) {
    let circles: Vec<(Entity, Position, Velocity, f32)> = world
        .positions_iter()
        .filter_map(|(entity, position)| {
            let circle = world.get_circle(*entity)?;
            let velocity = world.get_velocity(*entity)?;
            Some((*entity, *position, *velocity, circle.radius))
        })
        .collect();

    let mut position_updates: Vec<(Entity, f32, f32)> = Vec::new();
    let mut velocity_updates: Vec<(Entity, f32, f32)> = Vec::new();

    for i in 0..circles.len() {
        for j in (i + 1)..circles.len() {
            let (entity_a, position_a, velocity_a, radius_a) = circles[i];
            let (entity_b, position_b, velocity_b, radius_b) = circles[j];

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

                position_updates.push((entity_a, -correction_x, -correction_y));
                position_updates.push((entity_b, correction_x, correction_y));

                let relative_dx = velocity_b.dx - velocity_a.dx;
                let relative_dy = velocity_b.dy - velocity_a.dy;
                let normal_velocity = relative_dx * normal_x + relative_dy * normal_y;

                if normal_velocity < 0.0 {
                    let impulse = -(1.0 + bouncing_factor) * normal_velocity * 0.5;
                    let impulse_x = impulse * normal_x;
                    let impulse_y = impulse * normal_y;

                    velocity_updates.push((entity_a, -impulse_x, -impulse_y));
                    velocity_updates.push((entity_b, impulse_x, impulse_y));
                }
            }
        }
    }

    for (entity, correction_x, correction_y) in position_updates {
        if let Some(position) = world.get_position_mut(entity) {
            position.x += correction_x;
            position.y += correction_y;
        }
    }

    for (entity, delta_dx, delta_dy) in velocity_updates {
        if let Some(velocity) = world.get_velocity_mut(entity) {
            velocity.dx += delta_dx;
            velocity.dy += delta_dy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Circle, Position, Velocity};

    fn assert_f32_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {expected}, got {actual}"
        );
    }

    fn assert_position(world: &World2D, entity: Entity, expected_x: f32, expected_y: f32) {
        let pos = world.get_position(entity).unwrap();
        assert_f32_close(pos.x, expected_x);
        assert_f32_close(pos.y, expected_y);
    }

    fn assert_velocity(world: &World2D, entity: Entity, expected_dx: f32, expected_dy: f32) {
        let vel = world.get_velocity(entity).unwrap();
        assert_f32_close(vel.dx, expected_dx);
        assert_f32_close(vel.dy, expected_dy);
    }

    #[test]
    fn gravity_increases_downward_velocity() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 100.0, y: 100.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 0.0 });

        step(&mut world, 1.0, 100.0, 1000.0, 1000.0, 900.0, 1.0);

        assert_velocity(&world, entity, 0.0, 100.0);
    }

    #[test]
    fn gravity_moves_object_downward() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 100.0, y: 100.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 0.0 });

        step(&mut world, 1.0, 100.0, 1000.0, 1000.0, 900.0, 1.0);

        assert_position(&world, entity, 100.0, 200.0);
    }

    #[test]
    fn circle_bounces_on_ground() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 50.0, y: 90.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 50.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 500.0, 500.0, 100.0, 1.0);

        assert_position(&world, entity, 50.0, 90.0);
        assert_velocity(&world, entity, 0.0, -50.0);
    }

    #[test]
    fn object_falls_again_if_ground_moves_down() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 50.0, y: 90.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 0.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 500.0, 500.0, 100.0, 1.0);
        assert_position(&world, entity, 50.0, 90.0);

        step(&mut world, 1.0, 100.0, 500.0, 500.0, 200.0, 1.0);

        assert_velocity(&world, entity, 0.0, 100.0);
        assert_position(&world, entity, 50.0, 190.0);
    }

    #[test]
    fn circle_bounces_off_left_wall() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 10.0, y: 100.0 });
        world.add_velocity(entity, Velocity { dx: -50.0, dy: 0.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 500.0, 500.0, 400.0, 1.0);

        assert_position(&world, entity, 10.0, 100.0);
        assert_velocity(&world, entity, 50.0, 0.0);
    }

    #[test]
    fn circle_bounces_off_right_wall() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 490.0, y: 100.0 });
        world.add_velocity(entity, Velocity { dx: 50.0, dy: 0.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 500.0, 500.0, 400.0, 1.0);

        assert_position(&world, entity, 490.0, 100.0);
        assert_velocity(&world, entity, -50.0, 0.0);
    }

    #[test]
    fn circle_bounces_off_top_wall() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 100.0, y: 10.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: -50.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 500.0, 500.0, 400.0, 1.0);

        assert_position(&world, entity, 100.0, 10.0);
        assert_velocity(&world, entity, 0.0, 50.0);
    }

    #[test]
    fn bouncing_factor_scales_wall_bounce() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 10.0, y: 100.0 });
        world.add_velocity(entity, Velocity { dx: -40.0, dy: 0.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 500.0, 500.0, 400.0, 0.5);

        assert_position(&world, entity, 10.0, 100.0);
        assert_velocity(&world, entity, 20.0, 0.0);
    }

    #[test]
    fn bouncing_factor_scales_ground_bounce() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 100.0, y: 390.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 40.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 500.0, 500.0, 400.0, 0.5);

        assert_position(&world, entity, 100.0, 390.0);
        assert_velocity(&world, entity, 0.0, -20.0);
    }

    #[test]
    fn ground_uses_min_of_ground_y_and_viewport_height() {
        let mut world = World2D::new();
        let entity = world.spawn();

        world.add_position(entity, Position { x: 100.0, y: 285.0 });
        world.add_velocity(entity, Velocity { dx: 0.0, dy: 30.0 });
        world.add_circle(entity, Circle { radius: 10.0 });

        step(&mut world, 1.0, 0.0, 500.0, 300.0, 400.0, 1.0);

        assert_position(&world, entity, 100.0, 290.0);
        assert_velocity(&world, entity, 0.0, -30.0);
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

        step(&mut world, 0.0, 0.0, 1000.0, 1000.0, 900.0, 1.0);

        let pos_a = world.get_position(a).unwrap();
        let pos_b = world.get_position(b).unwrap();

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

        step(&mut world, 0.0, 0.0, 1000.0, 1000.0, 900.0, 1.0);

        assert_position(&world, a, 100.0, 100.0);
        assert_position(&world, b, 200.0, 100.0);
    }

    #[test]
    fn circles_bounce_head_on() {
        let mut world = World2D::new();

        let a = world.spawn();
        world.add_position(a, Position { x: 100.0, y: 100.0 });
        world.add_velocity(a, Velocity { dx: 10.0, dy: 0.0 });
        world.add_circle(a, Circle { radius: 20.0 });

        let b = world.spawn();
        world.add_position(b, Position { x: 130.0, y: 100.0 });
        world.add_velocity(b, Velocity { dx: -10.0, dy: 0.0 });
        world.add_circle(b, Circle { radius: 20.0 });

        step(&mut world, 0.0, 0.0, 1000.0, 1000.0, 900.0, 1.0);

        let vel_a = world.get_velocity(a).unwrap();
        let vel_b = world.get_velocity(b).unwrap();

        assert!((vel_a.dx + 10.0).abs() < 0.001);
        assert!((vel_b.dx - 10.0).abs() < 0.001);
    }

    #[test]
    fn circle_collision_uses_bouncing_factor() {
        let mut world = World2D::new();

        let a = world.spawn();
        world.add_position(a, Position { x: 100.0, y: 100.0 });
        world.add_velocity(a, Velocity { dx: 10.0, dy: 0.0 });
        world.add_circle(a, Circle { radius: 20.0 });

        let b = world.spawn();
        world.add_position(b, Position { x: 130.0, y: 100.0 });
        world.add_velocity(b, Velocity { dx: -10.0, dy: 0.0 });
        world.add_circle(b, Circle { radius: 20.0 });

        step(&mut world, 0.0, 0.0, 1000.0, 1000.0, 900.0, 0.5);

        let vel_a = world.get_velocity(a).unwrap();
        let vel_b = world.get_velocity(b).unwrap();

        assert!((vel_a.dx + 5.0).abs() < 0.001);
        assert!((vel_b.dx - 5.0).abs() < 0.001);
    }
}