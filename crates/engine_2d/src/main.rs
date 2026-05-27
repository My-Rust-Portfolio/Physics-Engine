mod app;
mod components;
mod physics;
mod scene;
mod world;

use app::Engine2DApp;
use eframe::egui;

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
