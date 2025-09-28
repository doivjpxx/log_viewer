use crate::file_manager::FileManager;
use crate::search_engine::SearchEngine;
use crate::types::{ApiError, FileInfo, FileChunk, SearchOptions, SearchResult};
use tauri::command;

#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
pub async fn open_file_dialog(app_handle: tauri::AppHandle) -> Result<Option<String>, ApiError> {
    use tauri_plugin_dialog::DialogExt;
    
    let file_path = app_handle
        .dialog()
        .file()
        .add_filter("Log files", &["log", "txt"])
        .add_filter("All files", &["*"])
        .set_title("Select a log file")
        .blocking_pick_file();
        
    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            Ok(Some(path_str))
        },
        None => Ok(None),
    }
}

#[command]
pub async fn get_file_info(path: String) -> Result<FileInfo, ApiError> {
    if path.is_empty() {
        return Err(ApiError {
            message: "File path cannot be empty".to_string(),
            code: "INVALID_PATH".to_string(),
        });
    }
    FileManager::get_file_info(&path).await
}

#[command]
pub async fn read_file_chunk(
    path: String,
    start_line: usize,
    end_line: usize,
) -> Result<FileChunk, ApiError> {
    if path.is_empty() {
        return Err(ApiError {
            message: "File path cannot be empty".to_string(),
            code: "INVALID_PATH".to_string(),
        });
    }
    if start_line > end_line {
        return Err(ApiError {
            message: "Start line cannot be greater than end line".to_string(),
            code: "INVALID_RANGE".to_string(),
        });
    }
    FileManager::read_file_chunk(&path, start_line, end_line).await
}

#[command]
pub async fn search_in_file(
    path: String,
    options: SearchOptions,
) -> Result<Vec<SearchResult>, ApiError> {
    if path.is_empty() {
        return Err(ApiError {
            message: "File path cannot be empty".to_string(),
            code: "INVALID_PATH".to_string(),
        });
    }
    if options.query.is_empty() {
        return Err(ApiError {
            message: "Search query cannot be empty".to_string(),
            code: "INVALID_QUERY".to_string(),
        });
    }
    SearchEngine::search_in_file(&path, options).await
}

#[command]
pub async fn get_lines_range(
    path: String,
    start_line: usize,
    end_line: usize,
) -> Result<Vec<String>, ApiError> {
    if path.is_empty() {
        return Err(ApiError {
            message: "File path cannot be empty".to_string(),
            code: "INVALID_PATH".to_string(),
        });
    }
    if start_line > end_line {
        return Err(ApiError {
            message: "Start line cannot be greater than end line".to_string(),
            code: "INVALID_RANGE".to_string(),
        });
    }
    FileManager::get_lines_range(&path, start_line, end_line).await
}