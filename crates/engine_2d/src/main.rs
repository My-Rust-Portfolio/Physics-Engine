mod components;
mod world;

use components::{Circle, Position, Velocity};
use eframe::egui;
use world::World2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tool {
    PlaceCircle,
}

struct Engine2DApp {
    world: World2D,
    selected_tool: Tool,
    next_radius: f32,
    next_velocity_x: f32,
    next_velocity_y: f32,
}

impl Engine2DApp {
    fn new() -> Self {
        Self {
            world: World2D::new(),
            selected_tool: Tool::PlaceCircle,
            next_radius: 20.0,
            next_velocity_x: 0.0,
            next_velocity_y: 0.0,
        }
    }

    fn spawn_circle(&mut self, x: f32, y: f32) {
        let entity = self.world.spawn();

        self.world.add_position(entity, Position { x, y });
        self.world.add_velocity(
            entity,
            Velocity {
                dx: self.next_velocity_x,
                dy: self.next_velocity_y,
            },
        );
        self.world.add_circle(
            entity,
            Circle {
                radius: self.next_radius,
            },
        );
    }
}

impl eframe::App for Engine2DApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.world.update_positions(1.0 / 60.0);

        egui::SidePanel::left("tools_panel").show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();

            ui.selectable_value(&mut self.selected_tool, Tool::PlaceCircle, "Place Circle");
            ui.separator();

            ui.heading("Spawn Settings");
            ui.add(egui::Slider::new(&mut self.next_radius, 5.0..=80.0).text("Radius"));
            ui.add(egui::Slider::new(&mut self.next_velocity_x, -200.0..=200.0).text("Velociy X"));
            ui.add(egui::Slider::new(&mut self.next_velocity_y, -200.0..=200.0).text("Velocity Y"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size_before_wrap();
            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::click());

            let rect = response.rect;
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(25, 25, 35));
            painter.rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(2.0, egui::Color32::from_rgb(60, 60, 80)),
                egui::StrokeKind::Inside,
            );

            if response.clicked() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let local_x = pointer_pos.x - rect.left();
                    let local_y = pointer_pos.y - rect.top();

                    match self.selected_tool {
                        Tool::PlaceCircle => self.spawn_circle(local_x, local_y),
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

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Engine 2D Editor",
        options,
        Box::new(|_cc| Ok(Box::new(Engine2DApp::new()))),
    )
}
