mod commands;
mod file_manager;
mod search_engine;
mod types;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            open_file_dialog,
            get_file_info,
            read_file_chunk,
            search_in_file
        ]);

    match builder.run(tauri::generate_context!()) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while running tauri application: {}", e);
            std::process::exit(1);
        }
    }
}
