use crate::types::{LogFormat, FormatDetectionResult, MultilineConfig, ParserError, ParsedLogLine};
use regex::Regex;
use std::collections::HashMap;

pub struct FormatDetector;

impl FormatDetector {
    /// Detect log format from sample lines
    pub fn detect_format(sample_lines: &[String]) -> Result<FormatDetectionResult, ParserError> {
        if sample_lines.is_empty() {
            return Err(ParserError {
                message: "No sample lines provided for format detection".to_string(),
                code: "NO_SAMPLE_LINES".to_string(),
            });
        }

        let mut format_scores: HashMap<LogFormat, f32> = HashMap::new();
        let detector = FormatDetector;
        
        // Analyze each line to determine format likelihood
        for line in sample_lines.iter().take(20) { // Analyze first 20 lines
            detector.analyze_line_for_json(&line, &mut format_scores);
            detector.analyze_line_for_structured(&line, &mut format_scores);
            detector.analyze_line_for_apache(&line, &mut format_scores);
            detector.analyze_line_for_nginx(&line, &mut format_scores);
            detector.analyze_line_for_syslog(&line, &mut format_scores);
        }
        
        // Default to plain text if no clear format detected
        if format_scores.is_empty() {
            format_scores.insert(LogFormat::Plain, 0.5);
        }

        // Find format with highest score
        let (detected_format, confidence) = format_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(format, score)| (format.clone(), *score))
            .unwrap();

        // Generate sample parsed lines
        let sample_parsed = detector.generate_sample_parsed_lines(sample_lines, &detected_format)?;
        
        // Suggest multiline configuration
        let multiline_config = detector.suggest_multiline_config(sample_lines, &detected_format);

        Ok(FormatDetectionResult {
            detected_format,
            confidence,
            sample_parsed_lines: sample_parsed,
            suggested_multiline_config: multiline_config,
        })
    }

    fn analyze_line_for_json(&self, line: &str, scores: &mut HashMap<LogFormat, f32>) {
        let trimmed = line.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            if let Ok(_) = serde_json::from_str::<serde_json::Value>(trimmed) {
                *scores.entry(LogFormat::JsonLines).or_insert(0.0) += 1.0;
            } else {
                // Partial JSON - might be multiline
                *scores.entry(LogFormat::JsonLines).or_insert(0.0) += 0.3;
            }
        }
    }

    fn analyze_line_for_structured(&self, line: &str, scores: &mut HashMap<LogFormat, f32>) {
        // Look for structured patterns like: TIMESTAMP LEVEL MESSAGE
        let structured_patterns = [
            r"^\d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2}.*\s+(ERROR|WARN|INFO|DEBUG|TRACE)\s+",
            r"^\[\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\].*\s+(ERROR|WARN|INFO|DEBUG|TRACE)\s+",
            r"^\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}.*",
        ];

        for pattern in &structured_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(line) {
                    *scores.entry(LogFormat::Plain).or_insert(0.0) += 0.8;
                    break;
                }
            }
        }
    }

    fn analyze_line_for_apache(&self, line: &str, scores: &mut HashMap<LogFormat, f32>) {
        // Common Log Format: host ident authuser [date] "request" status size
        let clf_pattern = r#"^(\S+) (\S+) (\S+) \[([^\]]+)\] "([^"]*)" (\d{3}) (\d+|-)"#;
        
        // Combined Log Format: CLF + "referer" "user-agent"
        let combined_pattern = r#"^(\S+) (\S+) (\S+) \[([^\]]+)\] "([^"]*)" (\d{3}) (\d+|-) "([^"]*)" "([^"]*)""#;

        if let Ok(regex) = Regex::new(combined_pattern) {
            if regex.is_match(line) {
                *scores.entry(LogFormat::Combined).or_insert(0.0) += 1.0;
                return;
            }
        }

        if let Ok(regex) = Regex::new(clf_pattern) {
            if regex.is_match(line) {
                *scores.entry(LogFormat::CommonLogFormat).or_insert(0.0) += 0.9;
            }
        }
    }

    fn analyze_line_for_nginx(&self, line: &str, scores: &mut HashMap<LogFormat, f32>) {
        // Nginx error log: 2019/04/18 14:30:50 [error] 12345#0: *1 message
        let nginx_error_pattern = r"^\d{4}/\d{2}/\d{2} \d{2}:\d{2}:\d{2} \[(error|warn|notice|info|debug)\] \d+#\d+:";
        
        // Nginx access log is similar to Apache combined
        if let Ok(regex) = Regex::new(nginx_error_pattern) {
            if regex.is_match(line) {
                *scores.entry(LogFormat::Nginx).or_insert(0.0) += 1.0;
            }
        }
    }

    fn analyze_line_for_syslog(&self, line: &str, scores: &mut HashMap<LogFormat, f32>) {
        // Syslog format: Oct 11 22:14:15 server program[pid]: message
        let syslog_pattern = r"^\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}\s+\S+\s+\S+(\[\d+\])?:";
        
        if let Ok(regex) = Regex::new(syslog_pattern) {
            if regex.is_match(line) {
                *scores.entry(LogFormat::Syslog).or_insert(0.0) += 1.0;
            }
        }
    }

    fn generate_sample_parsed_lines(
        &self,
        _sample_lines: &[String],
        _format: &LogFormat,
    ) -> Result<Vec<ParsedLogLine>, ParserError> {
        // For now, return empty vec - this would be implemented by specific parsers
        // In a full implementation, we'd delegate to the appropriate parser
        Ok(Vec::new())
    }

    fn suggest_multiline_config(&self, sample_lines: &[String], format: &LogFormat) -> Option<MultilineConfig> {
        match format {
            LogFormat::JsonLines => {
                // JSON might have multiline pretty-printed objects
                Some(MultilineConfig {
                    enabled: true,
                    start_pattern: Some(r"^\s*\{".to_string()),
                    continue_pattern: Some(r"^\s+".to_string()),
                    end_pattern: Some(r"^\s*\}".to_string()),
                    max_lines: 100,
                })
            },
            _ => {
                // Check for common multiline patterns like stack traces
                let has_stack_traces = sample_lines.iter().any(|line| {
                    line.trim().starts_with("at ") || 
                    line.trim().starts_with("Caused by:") ||
                    line.trim().starts_with("\tat ")
                });

                if has_stack_traces {
                    Some(MultilineConfig {
                        enabled: true,
                        start_pattern: Some(r"^\d{4}-\d{2}-\d{2}|^\w{3}\s+\d{1,2}|^\[".to_string()),
                        continue_pattern: Some(r"^\s+at\s+|^\s+\.\.\.\s+|^\s*Caused by:".to_string()),
                        end_pattern: None,
                        max_lines: 50,
                    })
                } else {
                    None
                }
            }
        }
    }
}