mod app;

use app::FfGuiApp;

use egui::widgets::{Button, Label};
use eframe::{egui, epaint::Vec2};
use std::process::Command;



fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "ffgui",
        options,
        Box::new(|_cc| Ok(Box::new(FfGuiApp::default()), )),
    )
}
