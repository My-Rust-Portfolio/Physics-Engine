use eframe::egui;

use crate::physics;
use crate::scene::{spawn_circle, find_circle_at_position};
use crate::world::World2D;
use crate::components::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tool {
    PlaceCircle,
}

pub struct Engine2DApp {
    world: World2D,
    selected_tool: Tool,
    next_radius: f32,
    ground_y: f32,
    gravity: f32,
    bouncing_factor: f32,
}

impl Engine2DApp {
    pub fn new() -> Self {
        Self {
            world: World2D::new(),
            selected_tool: Tool::PlaceCircle,
            next_radius: 20.0,
            ground_y: 500.0,
            gravity: 500.0,
            bouncing_factor: 0.45,
        }
    }
}

impl eframe::App for Engine2DApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("tools_panel").show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();

            ui.selectable_value(&mut self.selected_tool, Tool::PlaceCircle, "Place Circle");
            ui.label("Right-click a circle to delete it");
            ui.separator();

            ui.heading("Spawn Settings");
            ui.add(egui::Slider::new(&mut self.next_radius, 5.0..=80.0).text("Radius"));
            ui.separator();

            ui.heading("Scene");
            ui.add(egui::Slider::new(&mut self.ground_y, 100.0..=700.0).text("Ground Y"));
            ui.add(egui::Slider::new(&mut self.gravity, 0.0..=2000.0).text("Gravity"));
            ui.add(egui::Slider::new(&mut self.bouncing_factor, 0.0..=1.0).text("Bounciness"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size_before_wrap();
            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::click());

            let rect = response.rect;
            // do physics
            physics::step(
                &mut self.world,
                1.0 / 60.0,
                self.gravity,
                rect.width(),
                rect.height(),
                self.ground_y,
                self.bouncing_factor,
            );

            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(25, 25, 35));
            painter.rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(2.0, egui::Color32::from_rgb(60, 60, 80)),
                egui::StrokeKind::Inside,
            );

            let ground_start = egui::Pos2::new(rect.left(), rect.top() + self.ground_y);
            let ground_end = egui::Pos2::new(rect.right(), rect.top() + self.ground_y);

            painter.line_segment(
                [ground_start, ground_end],
                egui::Stroke::new(3.0, egui::Color32::from_rgb(180, 180, 100)),
            );

            if response.clicked() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let local_x = pointer_pos.x - rect.left();
                    let local_y = pointer_pos.y - rect.top();

                    match self.selected_tool {
                        Tool::PlaceCircle => {
                            spawn_circle(&mut self.world, local_x, local_y, self.next_radius);
                        }
                    }
                }
            }

            if response.clicked_by(egui::PointerButton::Secondary) {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let local_pos = Position {
                        x: pointer_pos.x - rect.min.x,
                        y: pointer_pos.y - rect.min.y,
                    };

                    if let Some(entity) = find_circle_at_position(&self.world, local_pos) {
                        self.world.despawn(entity);
                    }
                }
            }

            for (entity, position) in self.world.positions_iter() {
                if let Some(circle) = self.world.get_circle(*entity) {
                    let screen_pos =
                        egui::Pos2::new(rect.left() + position.x, rect.top() + position.y);

                    painter.circle_filled(
                        screen_pos,
                        circle.radius,
                        egui::Color32::from_rgb(100, 200, 255),
                    );
                }
            }
        });

        ctx.request_repaint();
    }
}
