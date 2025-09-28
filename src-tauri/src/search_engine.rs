use crate::types::{ApiError, SearchOptions, SearchResult, HighlightRange};
use anyhow::Result;
use regex::Regex;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};

pub struct SearchEngine;

impl SearchEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn search_in_file<P: AsRef<Path>>(
        path: P,
        options: SearchOptions,
    ) -> Result<Vec<SearchResult>, ApiError> {
        let file = tokio::fs::File::open(path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        
        let mut results = Vec::new();
        let mut line_number = 0;
        let mut all_lines: Vec<String> = Vec::new();

        // Read all lines first to support context lines
        while let Some(line) = lines.next_line().await? {
            all_lines.push(line);
        }

        // Prepare search pattern
        let pattern = if options.is_regex {
            match create_regex_pattern(&options.query, options.is_case_sensitive) {
                Ok(p) => p,
                Err(e) => return Err(e),
            }
        } else {
            match create_literal_pattern(&options.query, options.is_case_sensitive, options.is_whole_word) {
                Ok(p) => p,
                Err(e) => return Err(e),
            }
        };

        // Search through lines
        for (index, line) in all_lines.iter().enumerate() {
            line_number = index + 1;
            
            // Apply line range filtering if specified
            if let Some(start) = options.start_line {
                if line_number < start {
                    continue;
                }
            }
            if let Some(end) = options.end_line {
                if line_number > end {
                    break;
                }
            }
            
            if let Some(mut search_result) = search_in_line(line, line_number, &pattern) {
                // Add context lines if requested
                let context_lines = options.context_lines.unwrap_or(0);
                if context_lines > 0 {
                    search_result.context_before = Some(get_context_lines(
                        &all_lines,
                        index,
                        context_lines,
                        true,
                    ));
                    search_result.context_after = Some(get_context_lines(
                        &all_lines,
                        index,
                        context_lines,
                        false,
                    ));
                }
                
                results.push(search_result);
                
                if results.len() >= options.max_results {
                    break;
                }
            }
        }

        Ok(results)
    }

    /// Search within a single content string (for parsed content search)
    pub fn search_in_content(
        content: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, ApiError> {
        let pattern = if options.is_regex {
            create_regex_pattern(&options.query, options.is_case_sensitive)?
        } else {
            create_literal_pattern(&options.query, options.is_case_sensitive, options.is_whole_word)?
        };

        let mut results = Vec::new();
        
        if let Some(search_result) = search_in_line(content, 1, &pattern) {
            results.push(search_result);
        }

        Ok(results)
    }
}

fn search_in_line(line: &str, line_number: usize, pattern: &Regex) -> Option<SearchResult> {
    let mut highlights = Vec::new();
    
    for m in pattern.find_iter(line) {
        highlights.push(HighlightRange {
            start: m.start(),
            end: m.end(),
        });
    }

    if !highlights.is_empty() {
        Some(SearchResult {
            line_number,
            content: line.to_string(),
            highlights,
            context_before: None,
            context_after: None,
        })
    } else {
        None
    }
}

fn get_context_lines(
    all_lines: &[String],
    current_index: usize,
    context_count: usize,
    before: bool,
) -> Vec<String> {
    if context_count == 0 {
        return Vec::new();
    }

    let mut context = Vec::new();
    
    if before {
        let start = current_index.saturating_sub(context_count);
        for i in start..current_index {
            if i < all_lines.len() {
                context.push(all_lines[i].clone());
            }
        }
    } else {
        let start = current_index + 1;
        let end = std::cmp::min(start + context_count, all_lines.len());
        for i in start..end {
            context.push(all_lines[i].clone());
        }
    }

    context
}

fn create_regex_pattern(query: &str, case_sensitive: bool) -> Result<Regex, ApiError> {
    let mut builder = regex::RegexBuilder::new(query);
    builder.case_insensitive(!case_sensitive);
    
    builder.build().map_err(|e| ApiError {
        message: format!("Invalid regex pattern: {}", e),
        code: "INVALID_REGEX".to_string(),
    })
}

fn create_literal_pattern(query: &str, case_sensitive: bool, whole_word: bool) -> Result<Regex, ApiError> {
    let escaped = regex::escape(query);
    let pattern = if whole_word {
        format!(r"\b{}\b", escaped)
    } else {
        escaped
    };

    let mut builder = regex::RegexBuilder::new(&pattern);
    builder.case_insensitive(!case_sensitive);
    
    builder.build().map_err(|e| ApiError {
        message: format!("Failed to create search pattern: {}", e),
        code: "PATTERN_ERROR".to_string(),
    })
}