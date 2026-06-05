mod commands;

use std::fs;
use tauri::{App, Manager};

fn create_required_dirs(app: &mut App) {
    let appdata = app.path().app_data_dir().expect("Cant get app data dir");

    let plugins_dir = appdata.join("plugins");

    fs::create_dir_all(&plugins_dir).expect("Failed to create plugins dir");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            create_required_dirs(app);
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
