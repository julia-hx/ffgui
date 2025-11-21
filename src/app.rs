use egui::widgets::{Button, Label};
use eframe::{egui, epaint::Vec2};

pub struct FfGuiApp {
    name: String,
    age: u32,
}

impl Default for FfGuiApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for FfGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ffmpeg command launcher gui");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(Label::new(format!("Hello '{}', age {}", self.name, self.age)));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
        });
    }
}
