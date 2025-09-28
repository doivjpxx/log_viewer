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
        
        // Check if file exists and is readable
        if !path.exists() {
            return Err(ApiError {
                message: "File does not exist".to_string(),
                code: "FILE_NOT_FOUND".to_string(),
            });
        }

        if !path.is_file() {
            return Err(ApiError {
                message: "Path is not a file".to_string(),
                code: "NOT_A_FILE".to_string(),
            });
        }

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
        let line_count = Some(Self::count_lines(path).await?);

        Ok(FileInfo {
            path: path.to_string_lossy().to_string(),
            name,
            size,
            encoding,
            line_count,
            last_modified,
        })
    }

    pub async fn read_file_chunk<P: AsRef<Path>>(
        path: P,
        start_line: usize,
        end_line: usize,
    ) -> Result<FileChunk, ApiError> {
        let path = path.as_ref();
        
        // Validate file access
        if !path.exists() {
            return Err(ApiError {
                message: "File does not exist".to_string(),
                code: "FILE_NOT_FOUND".to_string(),
            });
        }

        let file = tokio::fs::File::open(path).await?;
        let reader = BufReader::with_capacity(64 * 1024, file); // 64KB buffer
        
        let mut content = Vec::new();
        let mut current_line = 0;
        let mut lines = reader.lines();
        let mut actual_end_line = start_line;

        // Skip to start line
        while current_line < start_line {
            match lines.next_line().await? {
                Some(_) => current_line += 1,
                None => {
                    // Reached EOF before start_line
                    return Ok(FileChunk {
                        start_line,
                        end_line: start_line,
                        content: Vec::new(),
                        total_lines: Some(current_line),
                    });
                }
            }
        }

        // Read lines in the specified range
        while current_line <= end_line && content.len() < (end_line - start_line + 1) {
            match lines.next_line().await? {
                Some(line) => {
                    // Handle very long lines (truncate if > 10KB)
                    let processed_line = if line.len() > 10240 {
                        format!("{}... [line truncated, {} chars total]", 
                               &line[..10240], line.len())
                    } else {
                        line
                    };
                    
                    content.push(processed_line);
                    actual_end_line = current_line;
                    current_line += 1;
                }
                None => break, // EOF
            }
        }

        Ok(FileChunk {
            start_line,
            end_line: actual_end_line,
            content,
            total_lines: None, // Can be computed separately if needed
        })
    }

    async fn detect_encoding<P: AsRef<Path>>(path: P) -> Result<String, ApiError> {
        let mut file = tokio::fs::File::open(path).await?;
        let mut buffer = vec![0; 8192]; // Read first 8KB for better detection
        
        let bytes_read = file.read(&mut buffer).await?;
        buffer.truncate(bytes_read);

        // Check for BOM (Byte Order Mark)
        if buffer.starts_with(&[0xEF, 0xBB, 0xBF]) {
            return Ok("UTF-8-BOM".to_string());
        } else if buffer.starts_with(&[0xFF, 0xFE]) {
            return Ok("UTF-16LE".to_string());
        } else if buffer.starts_with(&[0xFE, 0xFF]) {
            return Ok("UTF-16BE".to_string());
        } else if buffer.starts_with(&[0x00, 0x00, 0xFE, 0xFF]) {
            return Ok("UTF-32BE".to_string());
        } else if buffer.starts_with(&[0xFF, 0xFE, 0x00, 0x00]) {
            return Ok("UTF-32LE".to_string());
        }

        // Try UTF-8 validation
        match std::str::from_utf8(&buffer) {
            Ok(_) => {
                // Check if it looks like valid text (not binary)
                let valid_text_ratio = buffer.iter()
                    .filter(|&&b| b >= 32 || b == 9 || b == 10 || b == 13) // printable + tab/newline/CR
                    .count() as f32 / buffer.len() as f32;
                    
                if valid_text_ratio > 0.95 {
                    Ok("UTF-8".to_string())
                } else {
                    Ok("BINARY".to_string())
                }
            }
            Err(_) => {
                // Check for common Windows encodings
                if Self::is_likely_windows1252(&buffer) {
                    Ok("Windows-1252".to_string())
                } else if Self::is_likely_iso8859_1(&buffer) {
                    Ok("ISO-8859-1".to_string())
                } else {
                    Ok("UNKNOWN".to_string())
                }
            }
        }
    }

    fn is_likely_windows1252(buffer: &[u8]) -> bool {
        // Check for common Windows-1252 characters
        let mut score = 0;
        for &byte in buffer {
            match byte {
                0x80..=0x9F => score += 1, // Windows-1252 specific range
                _ => {}
            }
        }
        score > 0 && (score as f32 / buffer.len() as f32) < 0.1
    }

    fn is_likely_iso8859_1(buffer: &[u8]) -> bool {
        // Check for common ISO-8859-1 characters
        let mut valid_count = 0;
        for &byte in buffer {
            if byte <= 127 || byte >= 160 { // ASCII + high ISO range
                valid_count += 1;
            }
        }
        (valid_count as f32 / buffer.len() as f32) > 0.95
    }

    pub async fn count_lines<P: AsRef<Path>>(path: P) -> Result<usize, ApiError> {
        let file = tokio::fs::File::open(path).await?;
        let mut reader = BufReader::with_capacity(64 * 1024, file); // 64KB buffer
        let mut count = 0;
        let mut buffer = String::new();

        // Read in chunks for better performance
        loop {
            buffer.clear();
            let bytes_read = reader.read_line(&mut buffer).await?;
            if bytes_read == 0 {
                break; // EOF
            }
            count += 1;
            
            // Yield control periodically for large files
            if count % 10000 == 0 {
                tokio::task::yield_now().await;
            }
        }

        Ok(count)
    }
}