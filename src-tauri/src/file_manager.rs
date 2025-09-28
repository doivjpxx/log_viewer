use crate::types::{ApiError, FileInfo, FileChunk};
use anyhow::Result;
use std::path::Path;
use std::time::SystemTime;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

pub struct FileManager;

impl FileManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_file_info<P: AsRef<Path>>(path: P) -> Result<FileInfo, ApiError> {
        let path = path.as_ref();
        let metadata = tokio::fs::metadata(path).await?;
        
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let size = metadata.len();
        
        let last_modified = metadata
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_else(|_| "0".to_string());

        let encoding = Self::detect_encoding(path).await?;

        Ok(FileInfo {
            path: path.to_string_lossy().to_string(),
            name,
            size,
            encoding,
            line_count: None, // Will be computed lazily
            last_modified,
        })
    }

    pub async fn read_file_chunk<P: AsRef<Path>>(
        path: P,
        start_line: usize,
        end_line: usize,
    ) -> Result<FileChunk, ApiError> {
        let path = path.as_ref();
        let file = tokio::fs::File::open(path).await?;
        let reader = BufReader::new(file);
        
        let mut content = Vec::new();
        let mut current_line = 0;
        let mut lines = reader.lines();

        while current_line <= end_line {
            match lines.next_line().await? {
                Some(line) => {
                    if current_line >= start_line {
                        content.push(line);
                    }
                    current_line += 1;
                }
                None => break, // EOF
            }
        }

        Ok(FileChunk {
            start_line,
            end_line: start_line + content.len().saturating_sub(1),
            content,
            total_lines: None, // Can be computed if needed
        })
    }

    async fn detect_encoding<P: AsRef<Path>>(path: P) -> Result<String, ApiError> {
        let mut file = tokio::fs::File::open(path).await?;
        let mut buffer = vec![0; 1024]; // Read first 1KB for encoding detection
        
        let bytes_read = file.read(&mut buffer).await?;
        buffer.truncate(bytes_read);

        // Simple encoding detection - check for BOM or assume UTF-8
        if buffer.starts_with(&[0xEF, 0xBB, 0xBF]) {
            Ok("UTF-8".to_string())
        } else if buffer.starts_with(&[0xFF, 0xFE]) {
            Ok("UTF-16LE".to_string())
        } else if buffer.starts_with(&[0xFE, 0xFF]) {
            Ok("UTF-16BE".to_string())
        } else {
            // Try to decode as UTF-8
            match std::str::from_utf8(&buffer) {
                Ok(_) => Ok("UTF-8".to_string()),
                Err(_) => Ok("ASCII".to_string()), // Fallback
            }
        }
    }

    pub async fn count_lines<P: AsRef<Path>>(path: P) -> Result<usize, ApiError> {
        let file = tokio::fs::File::open(path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut count = 0;

        while lines.next_line().await?.is_some() {
            count += 1;
        }

        Ok(count)
    }
}