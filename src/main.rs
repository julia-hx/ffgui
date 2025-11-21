mod app;
mod commands;

use app::FfGuiApp;
use eframe;

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
