use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

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

// Parser-related types for Week 7 implementation

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LogFormat {
    Plain,
    JsonLines,
    CustomRegex(String),
    CommonLogFormat, // Apache Common Log
    Combined,        // Apache Combined Log
    Nginx,
    Syslog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    pub format: LogFormat,
    pub multiline: MultilineConfig,
    pub timestamp_format: Option<String>,
    pub custom_fields: Vec<FieldExtractor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilineConfig {
    pub enabled: bool,
    pub start_pattern: Option<String>,
    pub continue_pattern: Option<String>,
    pub end_pattern: Option<String>,
    pub max_lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldExtractor {
    pub name: String,
    pub regex: String,
    pub data_type: FieldType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Timestamp,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParsedLogLine {
    pub raw_content: String,
    pub line_number: u64,
    pub byte_offset: u64,
    pub timestamp: Option<DateTime<Utc>>,
    pub level: Option<LogLevel>,
    pub message: String,
    pub fields: HashMap<String, serde_json::Value>,
    pub is_multiline: bool,
    pub multiline_group_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub content: String,
    pub line_number: u64,
    pub byte_offset: u64,
    pub is_multiline: bool,
    pub multiline_group_id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct FormatDetectionResult {
    pub detected_format: LogFormat,
    pub confidence: f32,
    pub sample_parsed_lines: Vec<ParsedLogLine>,
    pub suggested_multiline_config: Option<MultilineConfig>,
}

#[derive(Debug, Serialize)]
pub struct ParserError {
    pub message: String,
    pub code: String,
}

impl From<regex::Error> for ParserError {
    fn from(error: regex::Error) -> Self {
        ParserError {
            message: error.to_string(),
            code: "REGEX_ERROR".to_string(),
        }
    }
}

impl From<serde_json::Error> for ParserError {
    fn from(error: serde_json::Error) -> Self {
        ParserError {
            message: error.to_string(),
            code: "JSON_PARSE_ERROR".to_string(),
        }
    }
}

impl From<ParserError> for ApiError {
    fn from(error: ParserError) -> Self {
        ApiError {
            message: error.message,
            code: error.code,
        }
    }
}