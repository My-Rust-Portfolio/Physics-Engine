mod components;
mod world;

use eframe::egui;
use world::World2D;

struct Engine2DApp {
    world: World2D,
}

impl Engine2DApp {
    fn new() -> Self {
        Self {
            world: World2D::new(),
        }
    }
}

impl eframe::App for Engine2DApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

    // run_native starts the window and takes control of the main thread
    eframe::run_native(
        "Engine 2D Editor",
        options,
        Box::new(|_cc| Ok(Box::new(Engine2DApp::new()))),
    )
}
