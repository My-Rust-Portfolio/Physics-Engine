mod components;
mod world;

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
}

impl eframe::App for Engine2DApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            ui.heading("Physics Engine 2D");
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
