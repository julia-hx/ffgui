use std::path::PathBuf;
use egui::widgets::{Button, Label};
use eframe::{egui, epaint::Vec2};
use egui_file_dialog::FileDialog;

use crate::commands::{
	CommandType, 
	to_mp3, 
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
	source_paths: Vec<Option<PathBuf>>,

	command: CommandType,
	quality: Quality,
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
			source_paths: Vec::new(),

			command: CommandType::ToMp3,
			quality: Quality::default(),
        }
    }
}

impl eframe::App for FfGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ffmpeg command launcher");
			ui.weak("");

			// picker
			ui.strong("Select source");
			ui.horizontal(|ui| {
				if ui.button("File").clicked() {
					self.file_dialog.pick_file();
					self.picking_file = true;
					self.picking_directory = false;
				}
				if ui.button("Folder").clicked() {
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

			// if we have picked something:
			if self.source_file_picked || self.source_directory_picked {
				let path_string = self.source_path.clone().expect("no valid path").into_os_string().into_string().unwrap();
				if self.source_file_picked {
					ui.strong("Source file");
					ui.weak(format!("{}", path_string));
				} else if self.source_directory_picked {
					// TODO: collect file paths in folder
					let directory = std::fs::read_dir(&path_string).unwrap();
					let mut total_rows = 0;
					let mut path_strings = Vec::new();
					for entry in directory {
						let path = Some(entry.as_ref().expect("no valid path").path(), );
						self.source_paths.push(path);

						// get strings into separate collection?
						let ps = entry.expect("no valid path").path().into_os_string().into_string().unwrap();
						path_strings.push(ps);
						total_rows = total_rows + 1;
					}
					
					let text_style = egui::TextStyle::Body;
					let row_height = ui.text_style_height(&text_style);
					//let total_rows = 100;
					ui.strong("Files in source folder");
					egui::ScrollArea::vertical()
						.max_height(120.0)
						.max_width(800.0)
						.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
						.show_rows(ui, row_height, total_rows, |ui, row_range| {
							for n in row_range {
								ui.label(path_strings[n].to_owned());
							}
						}
					);
				}
				
				ui.strong("Command config");
				let mut current_command = self.command;
				egui::ComboBox::from_label("Command")
					.selected_text(format!("{:?}", current_command))
					.show_ui(ui, |ui| {
						ui.selectable_value(&mut current_command, CommandType::ToMp3, "To Mp3");
						ui.selectable_value(&mut current_command, CommandType::ToWav, "To Wav");
						ui.selectable_value(&mut current_command, CommandType::ResizeVideo, "Resize Video");
						ui.selectable_value(&mut current_command, CommandType::CropVideo, "Crop Video");
					}
				);
				if self.command != current_command { self.command = current_command; }	

				if self.command == CommandType::ToMp3 {
					let mut q = self.quality;
					egui::ComboBox::from_label("Quality")
						.selected_text(format!("{:?}", q))
						.show_ui(ui, |ui| {
							ui.selectable_value(&mut q, Quality::High, "High");
							ui.selectable_value(&mut q, Quality::Medium, "Medium");
							ui.selectable_value(&mut q, Quality::Low, "Low");
							ui.selectable_value(&mut q, Quality::Broken, "Broken");
						}
					);
					if self.quality != q { self.quality = q; }
				}
				
				ui.strong("Run");
				
				if ui.button("Go!").clicked() {
					if self.source_file_picked {
						match self.command {
							CommandType::ToMp3  => { to_mp3(&self.source_path, self.quality); },
							CommandType::ToWav => {},
							CommandType::ResizeVideo => {},
							CommandType::CropVideo => {},
						}
					}
					else if self.source_directory_picked {
						let paths = self.source_paths.clone();
						for path in paths {
							match self.command {
								CommandType::ToMp3  => { to_mp3(&path, self.quality); },
								CommandType::ToWav => {},
								CommandType::ResizeVideo => {},
								CommandType::CropVideo => {},
							}
						}
					}
				}
			}
        });
    }
}
