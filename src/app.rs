use std::path::PathBuf;
use egui::widgets::{Button, Label};
use eframe::{egui, epaint::Vec2};
use egui_file_dialog::FileDialog;

use crate::commands::{
	CommandType, 
	to_mp3, 
	ToMp3Arguments, 
	Quality
};

pub struct FfGuiApp {
	//arg: String,
	file_dialog: FileDialog,
	picking_file: bool,
	picking_directory: bool,
	source_file_picked: bool,
	source_directory_picked: bool,
	source_path: Option<PathBuf>,

	command: CommandType,
	mp3_args: ToMp3Arguments,
}

impl Default for FfGuiApp {
    fn default() -> Self {
        Self {
			//arg: "".to_owned(),
			file_dialog: FileDialog::new(),
			picking_file: false,
			picking_directory: false,
			source_file_picked: false,
			source_directory_picked: false,
			source_path: None,

			command: CommandType::ToMp3,
			mp3_args: ToMp3Arguments::default(),
        }
    }
}

impl eframe::App for FfGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ffmpeg command launcher");
			ui.weak("");

			// picker
			ui.strong("Select source:");
			ui.horizontal(|ui| {
				if ui.button("Open Source File").clicked() {
					self.file_dialog.pick_file();
					self.picking_file = true;
					self.picking_directory = false;
				}
				if ui.button("Select Source Folder").clicked() {
					self.file_dialog.pick_directory();
					self.picking_directory = true;
					self.picking_file = false;
				}
			});
			
			// picker logic
			self.file_dialog.update(ctx);
			if let Some(path) = self.file_dialog.take_picked() {
				self.source_path = Some(path.to_path_buf());
				if self.picking_file {
					self.source_file_picked = true;
					self.source_directory_picked = false;
					self.picking_file = false;
				}
				if self.picking_directory {
					self.source_directory_picked = true;
					self.source_file_picked = false;
					self.picking_directory = false;
				}
            }

			if self.source_file_picked {
				let path_string = self.source_path.clone().expect("no valid path").into_os_string().into_string().unwrap();
				ui.strong("Source File:");
				ui.weak(format!("{}", path_string));

				ui.strong("Command to run:");
				let mut current_command = self.command;
				egui::ComboBox::from_id_salt(1)
					.selected_text(format!("{:?}", current_command))
					.show_ui(ui, |ui| {
						ui.selectable_value(&mut current_command, CommandType::ToMp3, "To Mp3");
						ui.selectable_value(&mut current_command, CommandType::ToWav, "To Wav");
						ui.selectable_value(&mut current_command, CommandType::ResizeVideo, "Resize Video");
						ui.selectable_value(&mut current_command, CommandType::CropVideo, "Crop Video");
					}
				);
				if self.command != current_command { self.command = current_command; }	

				ui.strong("Command config:");
				if self.command == CommandType::ToMp3 {
					ui.checkbox(&mut self.mp3_args.constant_bitrate, "Constant bitrate");
					let mut q = self.mp3_args.quality;
					egui::ComboBox::from_label("Quality")
						.selected_text(format!("{:?}", q))
						.show_ui(ui, |ui| {
							ui.selectable_value(&mut q, Quality::High, "High");
							ui.selectable_value(&mut q, Quality::Medium, "Medium");
							ui.selectable_value(&mut q, Quality::Low, "Low");
						}
					);
					if self.mp3_args.quality != q { self.mp3_args.quality = q; }
				}
				
				ui.strong("Run command:");
				if ui.button("Go!").clicked() {
					match self.command {
						CommandType::ToMp3  => { to_mp3(&path_string, self.mp3_args); },
						CommandType::ToWav => {},
						CommandType::ResizeVideo => {},
						CommandType::CropVideo => {},
					}
				}
			}
        });
    }
}
