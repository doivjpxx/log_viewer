use crate::file_manager::FileManager;
use crate::search_engine::SearchEngine;
use crate::types::{ApiError, FileInfo, FileChunk, SearchOptions, SearchResult};
use tauri::command;

#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
pub async fn open_file_dialog() -> Result<Option<String>, ApiError> {
    // Placeholder implementation
    // This will be properly implemented with tauri-plugin-dialog
    Ok(Some("placeholder_path".to_string()))
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