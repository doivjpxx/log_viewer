use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub encoding: String,
    pub line_count: Option<usize>,
    pub last_modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunk {
    pub start_line: usize,
    pub end_line: usize,
    pub content: Vec<String>,
    pub total_lines: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    pub query: String,
    pub is_regex: bool,
    pub is_case_sensitive: bool,
    pub is_whole_word: bool,
    pub max_results: usize,
    pub context_lines: Option<usize>,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub line_number: usize,
    pub content: String,
    pub highlights: Vec<HighlightRange>,
    pub context_before: Option<Vec<String>>,
    pub context_after: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: String,
}

impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> Self {
        ApiError {
            message: error.to_string(),
            code: "IO_ERROR".to_string(),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        ApiError {
            message: error.to_string(),
            code: "GENERAL_ERROR".to_string(),
        }
    }
}