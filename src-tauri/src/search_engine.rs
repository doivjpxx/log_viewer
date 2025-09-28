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

        while let Some(line) = lines.next_line().await? {
            line_number += 1;
            
            if let Some(search_result) = search_in_line(&line, line_number, &pattern) {
                results.push(search_result);
                
                if results.len() >= options.max_results {
                    break;
                }
            }
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
        })
    } else {
        None
    }
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