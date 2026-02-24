// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_test_command() {
	println!("test command!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, run_test_command])
        .on_window_event(|window, event| {
            use tauri::{DragDropEvent, Emitter, WindowEvent};
            if let WindowEvent::DragDrop(drag) = event {
                match drag {
                    DragDropEvent::Enter { paths, .. } => {
                        let payload: Vec<String> = paths
                            .into_iter()
                            .map(|p| p.to_string_lossy().into_owned())
                            .collect();
                        let _ = window.emit("file-drop-hover", payload);
                    }
                    DragDropEvent::Over { .. } => {
                        // Keep hover state true; no path update.
                        let _ = window.emit("file-drop-hover", Vec::<String>::new());
                    }
                    DragDropEvent::Drop { paths, .. } => {
                        let payload: Vec<String> = paths
                            .into_iter()
                            .map(|p| p.to_string_lossy().into_owned())
                            .collect();
                        let _ = window.emit("file-drop", payload);
                    }
                    DragDropEvent::Leave => {
                        let _ = window.emit("file-drop-cancelled", ());
                    }
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
