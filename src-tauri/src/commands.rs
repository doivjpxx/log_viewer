use crate::file_manager::FileManager;
use crate::search_engine::SearchEngine;
use crate::format_detector::FormatDetector;
use crate::parser_engine::ParserEngine;
use crate::types::{ApiError, FileInfo, FileChunk, SearchOptions, SearchResult, ParserConfig, FormatDetectionResult, ParsedLogLine};
use tauri::command;
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

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

// Parser-related commands for Week 7 implementation

#[command]
pub async fn detect_log_format(path: String, sample_size: Option<usize>) -> Result<FormatDetectionResult, ApiError> {
    if path.is_empty() {
        return Err(ApiError {
            message: "File path cannot be empty".to_string(),
            code: "INVALID_PATH".to_string(),
        });
    }

    let sample_size = sample_size.unwrap_or(20);
    let lines = FileManager::get_lines_range(&path, 0, sample_size).await?;
    let detection_result = FormatDetector::detect_format(&lines)?;
    
    Ok(detection_result)
}

#[command]
pub async fn parse_lines_with_config(
    lines: Vec<String>,
    config: ParserConfig,
    start_line_number: u64,
    start_byte_offset: u64,
) -> Result<Vec<ParsedLogLine>, ApiError> {
    let mut parser = ParserEngine::new(config)?;
    let parsed_lines = parser.parse_lines(&lines, start_line_number, start_byte_offset)?;
    
    Ok(parsed_lines)
}

#[command]
pub async fn get_parsed_lines_range(
    path: String,
    start_line: usize,
    end_line: usize,
    config: ParserConfig,
) -> Result<Vec<ParsedLogLine>, ApiError> {
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

    let lines = FileManager::get_lines_range(&path, start_line, end_line).await?;
    let mut parser = ParserEngine::new(config)?;
    let parsed_lines = parser.parse_lines(&lines, start_line as u64, 0)?;
    
    Ok(parsed_lines)
}

// Week 8 Advanced Parser Integration Commands

#[derive(Serialize, Deserialize)]
pub struct ParsedSearchResult {
    pub parsed_line: ParsedLogLine,
    pub search_matches: Vec<SearchResult>,
    pub highlighted_content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParserPerformanceMetrics {
    pub lines_processed: usize,
    pub processing_time_ms: u64,
    pub lines_per_second: f64,
    pub multiline_groups: usize,
    pub parse_errors: usize,
    pub memory_usage_bytes: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ExportOptions {
    pub format: String, // "json", "csv", "plain"
    pub include_raw_content: bool,
    pub include_parsed_fields: bool,
    pub include_metadata: bool,
    pub max_lines: Option<usize>,
}

#[command]
pub async fn parse_file_batch(
    path: String,
    config: ParserConfig,
    batch_size: usize,
    start_line: usize,
    max_lines: Option<usize>,
) -> Result<(Vec<ParsedLogLine>, ParserPerformanceMetrics), ApiError> {
    let start_time = Instant::now();
    let mut all_parsed_lines = Vec::new();
    let mut multiline_groups = 0;
    let mut parse_errors = 0;
    let mut lines_processed = 0;

    let total_lines = max_lines.unwrap_or(usize::MAX);
    let mut current_line = start_line;
    
    // Create parser once for efficiency
    let mut parser = ParserEngine::new(config)?;

    while lines_processed < total_lines {
        let end_line = std::cmp::min(current_line + batch_size, start_line + total_lines);
        if current_line >= end_line {
            break;
        }

        match FileManager::get_lines_range(&path, current_line, end_line).await {
            Ok(lines) => {
                match parser.parse_lines(&lines, current_line as u64, 0) {
                    Ok(parsed) => {
                        multiline_groups += parsed.iter().filter(|p| p.is_multiline).count();
                        all_parsed_lines.extend(parsed);
                    }
                    Err(_) => parse_errors += 1,
                }
                lines_processed += lines.len();
                current_line = end_line;
            }
            Err(_) => {
                parse_errors += 1;
                break;
            }
        }

        // Yield control to avoid blocking
        tokio::task::yield_now().await;
    }

    let processing_time = start_time.elapsed();
    let lines_per_second = if processing_time.as_secs_f64() > 0.0 {
        lines_processed as f64 / processing_time.as_secs_f64()
    } else {
        0.0
    };

    let metrics = ParserPerformanceMetrics {
        lines_processed,
        processing_time_ms: processing_time.as_millis() as u64,
        lines_per_second,
        multiline_groups,
        parse_errors,
        memory_usage_bytes: all_parsed_lines.len() * std::mem::size_of::<ParsedLogLine>(),
    };

    Ok((all_parsed_lines, metrics))
}

#[command]
pub async fn search_in_parsed_content(
    path: String,
    parser_config: ParserConfig,
    search_options: SearchOptions,
    start_line: usize,
    end_line: usize,
) -> Result<Vec<ParsedSearchResult>, ApiError> {
    // Get and parse lines
    let lines = FileManager::get_lines_range(&path, start_line, end_line).await?;
    let mut parser = ParserEngine::new(parser_config)?;
    let parsed_lines = parser.parse_lines(&lines, start_line as u64, 0)?;

    let mut results = Vec::new();

    for parsed_line in parsed_lines {
        // Search in both raw content and parsed message
        let search_targets = vec![
            parsed_line.raw_content.clone(),
            parsed_line.message.clone(),
        ];

        let mut line_matches = Vec::new();
        let mut highlighted_content = parsed_line.raw_content.clone();

        for target in search_targets {
            if let Ok(matches) = SearchEngine::search_in_content(&target, &search_options) {
                line_matches.extend(matches);
            }
        }

                // Add field-specific searches for structured data
        for (field_name, field_value) in &parsed_line.fields {
            if let Some(field_str) = field_value.as_str() {
                if let Ok(matches) = SearchEngine::search_in_content(field_str, &search_options) {
                    // Adjust matches to indicate they're from a specific field
                    for mut match_result in matches {
                        let field_context = format!("[{}]", field_name);
                        if let Some(existing_context) = match_result.context_before {
                            match_result.context_before = Some([vec![field_context], existing_context].concat());
                        } else {
                            match_result.context_before = Some(vec![field_context]);
                        }
                        line_matches.push(match_result);
                    }
                }
            }
        }

        // Create highlighted content for UI display
        if !line_matches.is_empty() {
            // Apply highlighting to the content
            highlighted_content = apply_search_highlighting(&highlighted_content, &search_options.query);
            
            results.push(ParsedSearchResult {
                parsed_line,
                search_matches: line_matches,
                highlighted_content,
            });
        }
    }

    Ok(results)
}

#[command]
pub async fn export_parsed_content(
    path: String,
    parser_config: ParserConfig,
    export_options: ExportOptions,
    start_line: usize,
    end_line: usize,
) -> Result<String, ApiError> {
    let lines = FileManager::get_lines_range(&path, start_line, end_line).await?;
    let mut parser = ParserEngine::new(parser_config)?;
    let parsed_lines = parser.parse_lines(&lines, start_line as u64, 0)?;

    let limited_lines = if let Some(max) = export_options.max_lines {
        parsed_lines.into_iter().take(max).collect::<Vec<_>>()
    } else {
        parsed_lines
    };

    match export_options.format.as_str() {
        "json" => export_as_json(&limited_lines, &export_options),
        "csv" => export_as_csv(&limited_lines, &export_options),
        "plain" => export_as_plain_text(&limited_lines, &export_options),
        _ => Err(ApiError {
            message: "Unsupported export format".to_string(),
            code: "INVALID_FORMAT".to_string(),
        })
    }
}

#[command]
pub async fn validate_parser_config(config: ParserConfig) -> Result<Vec<String>, ApiError> {
    let mut warnings = Vec::new();

    // Validate multiline configuration
    if config.multiline.enabled {
        if config.multiline.start_pattern.is_none() && config.multiline.continue_pattern.is_none() {
            warnings.push("Multiline enabled but no patterns specified".to_string());
        }

        if let Some(ref pattern) = config.multiline.start_pattern {
            if let Err(_) = regex::Regex::new(pattern) {
                warnings.push(format!("Invalid start pattern: {}", pattern));
            }
        }

        if let Some(ref pattern) = config.multiline.continue_pattern {
            if let Err(_) = regex::Regex::new(pattern) {
                warnings.push(format!("Invalid continue pattern: {}", pattern));
            }
        }

        if config.multiline.max_lines > 1000 {
            warnings.push("Very high max_lines may impact performance".to_string());
        }
    }

    // Validate custom field extractors
    for (index, field) in config.custom_fields.iter().enumerate() {
        if let Err(_) = regex::Regex::new(&field.regex) {
            warnings.push(format!("Invalid regex in field {}: {}", index, field.regex));
        }
    }

    Ok(warnings)
}

// Helper functions for export functionality

fn export_as_json(lines: &[ParsedLogLine], options: &ExportOptions) -> Result<String, ApiError> {
    let export_data: Vec<_> = lines.iter().map(|line| {
        let mut obj = serde_json::Map::new();

        if options.include_metadata {
            obj.insert("line_number".to_string(), serde_json::Value::Number(line.line_number.into()));
            obj.insert("byte_offset".to_string(), serde_json::Value::Number(line.byte_offset.into()));
            obj.insert("is_multiline".to_string(), serde_json::Value::Bool(line.is_multiline));
            if let Some(group_id) = line.multiline_group_id {
                obj.insert("multiline_group_id".to_string(), serde_json::Value::Number(group_id.into()));
            }
        }

        if options.include_raw_content {
            obj.insert("raw_content".to_string(), serde_json::Value::String(line.raw_content.clone()));
        }

        if options.include_parsed_fields {
            obj.insert("message".to_string(), serde_json::Value::String(line.message.clone()));
            if let Some(ref timestamp) = line.timestamp {
                obj.insert("timestamp".to_string(), serde_json::Value::String(timestamp.clone()));
            }
            if let Some(ref level) = line.level {
                obj.insert("level".to_string(), serde_json::Value::String(format!("{:?}", level)));
            }
            if !line.fields.is_empty() {
                obj.insert("fields".to_string(), serde_json::to_value(&line.fields).unwrap_or(serde_json::Value::Null));
            }
        }

        serde_json::Value::Object(obj)
    }).collect();

    serde_json::to_string_pretty(&export_data)
        .map_err(|e| ApiError {
            message: format!("JSON export failed: {}", e),
            code: "EXPORT_ERROR".to_string(),
        })
}

fn export_as_csv(lines: &[ParsedLogLine], options: &ExportOptions) -> Result<String, ApiError> {
    let mut csv_content = String::new();
    
    // CSV Header
    let mut headers = Vec::new();
    if options.include_metadata {
        headers.extend_from_slice(&["line_number", "byte_offset", "is_multiline", "multiline_group_id"]);
    }
    if options.include_parsed_fields {
        headers.extend_from_slice(&["timestamp", "level", "message"]);
    }
    if options.include_raw_content {
        headers.push("raw_content");
    }
    csv_content.push_str(&headers.join(","));
    csv_content.push('\n');

    // CSV Data
    for line in lines {
        let mut values = Vec::new();
        
        if options.include_metadata {
            values.push(line.line_number.to_string());
            values.push(line.byte_offset.to_string());
            values.push(line.is_multiline.to_string());
            values.push(line.multiline_group_id.map_or("".to_string(), |id| id.to_string()));
        }
        
        if options.include_parsed_fields {
            values.push(line.timestamp.as_ref().unwrap_or(&String::new()).clone());
            values.push(line.level.as_ref().map_or(String::new(), |l| format!("{:?}", l)));
            values.push(escape_csv_field(&line.message));
        }
        
        if options.include_raw_content {
            values.push(escape_csv_field(&line.raw_content));
        }

        csv_content.push_str(&values.join(","));
        csv_content.push('\n');
    }

    Ok(csv_content)
}

fn export_as_plain_text(lines: &[ParsedLogLine], options: &ExportOptions) -> Result<String, ApiError> {
    let mut content = String::new();
    
    for line in lines {
        if options.include_parsed_fields && line.timestamp.is_some() && line.level.is_some() {
            // Format as structured log entry
            content.push_str(&format!(
                "[{}] [{}] {}\n",
                line.timestamp.as_ref().unwrap(),
                line.level.as_ref().map_or("INFO".to_string(), |l| format!("{:?}", l)),
                line.message
            ));
        } else if options.include_raw_content {
            content.push_str(&line.raw_content);
            content.push('\n');
        } else {
            content.push_str(&line.message);
            content.push('\n');
        }

        if options.include_metadata {
            content.push_str(&format!("  # Line {}, Offset {}\n", line.line_number, line.byte_offset));
        }
    }

    Ok(content)
}

fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

fn apply_search_highlighting(content: &str, query: &str) -> String {
    // Simple highlighting - in a real implementation, this would be more sophisticated
    content.replace(query, &format!("**{}**", query))
}