use crate::types::{
    LogFormat, ParserConfig, ParsedLogLine, LogEntry, ParserError, LogLevel
};
use regex::Regex;
use std::collections::HashMap;
use serde_json::Value;

pub trait LogFormatParser: Send + Sync {
    fn parse(&self, content: &str) -> Result<ParsedLogLine, ParserError>;
}

pub struct ParserEngine {
    config: ParserConfig,
    multiline_buffer: Vec<String>,
    multiline_group_counter: u64,
    format_specific_parser: Box<dyn LogFormatParser + Send + Sync>,
}

impl ParserEngine {
    pub fn new(config: ParserConfig) -> Result<Self, ParserError> {
        let format_parser = Self::create_format_parser(&config.format)?;
        
        Ok(Self {
            config,
            multiline_buffer: Vec::new(),
            multiline_group_counter: 0,
            format_specific_parser: format_parser,
        })
    }

    pub fn parse_lines(
        &mut self,
        lines: &[String],
        start_line_number: u64,
        start_byte_offset: u64,
    ) -> Result<Vec<ParsedLogLine>, ParserError> {
        let mut parsed_lines = Vec::new();
        let mut current_line_number = start_line_number;
        let mut current_byte_offset = start_byte_offset;

        for line in lines {
            if self.config.multiline.enabled {
                if let Some(complete_entry) = self.process_multiline_entry(
                    line,
                    current_line_number,
                    current_byte_offset,
                )? {
                    let parsed = self.parse_single_entry(&complete_entry)?;
                    parsed_lines.push(parsed);
                }
            } else {
                let entry = LogEntry {
                    content: line.clone(),
                    line_number: current_line_number,
                    byte_offset: current_byte_offset,
                    is_multiline: false,
                    multiline_group_id: None,
                };
                
                let parsed = self.parse_single_entry(&entry)?;
                parsed_lines.push(parsed);
            }

            current_line_number += 1;
            current_byte_offset += line.len() as u64 + 1; // +1 for newline
        }

        // Process any remaining multiline buffer
        if !self.multiline_buffer.is_empty() {
            let entry = self.flush_multiline_buffer(current_line_number, current_byte_offset)?;
            if let Some(complete_entry) = entry {
                let parsed = self.parse_single_entry(&complete_entry)?;
                parsed_lines.push(parsed);
            }
        }

        Ok(parsed_lines)
    }

    fn process_multiline_entry(
        &mut self,
        line: &str,
        line_number: u64,
        byte_offset: u64,
    ) -> Result<Option<LogEntry>, ParserError> {
        let is_start_line = self.is_multiline_start(line)?;
        let is_continue_line = self.is_multiline_continue(line)?;

        if is_start_line {
            // Flush previous multiline entry if exists
            let previous_entry = if !self.multiline_buffer.is_empty() {
                self.flush_multiline_buffer(line_number - 1, byte_offset)?
            } else {
                None
            };

            // Start new multiline entry
            self.multiline_buffer.push(line.to_string());
            self.multiline_group_counter += 1;

            Ok(previous_entry)
        } else if is_continue_line && !self.multiline_buffer.is_empty() {
            // Continue current multiline entry
            self.multiline_buffer.push(line.to_string());
            
            // Check max lines limit
            if self.multiline_buffer.len() >= self.config.multiline.max_lines {
                Ok(self.flush_multiline_buffer(line_number, byte_offset)?)
            } else {
                Ok(None)
            }
        } else {
            // Single line or end of multiline
            if !self.multiline_buffer.is_empty() {
                let entry = self.flush_multiline_buffer(line_number - 1, byte_offset)?;
                
                // Process current line as new single line will be handled in next iteration
                Ok(entry)
            } else {
                // Single line
                let entry = LogEntry {
                    content: line.to_string(),
                    line_number,
                    byte_offset,
                    is_multiline: false,
                    multiline_group_id: None,
                };
                Ok(Some(entry))
            }
        }
    }

    fn is_multiline_start(&self, line: &str) -> Result<bool, ParserError> {
        if let Some(pattern) = &self.config.multiline.start_pattern {
            let regex = Regex::new(pattern)?;
            Ok(regex.is_match(line))
        } else {
            Ok(false)
        }
    }

    fn is_multiline_continue(&self, line: &str) -> Result<bool, ParserError> {
        if let Some(pattern) = &self.config.multiline.continue_pattern {
            let regex = Regex::new(pattern)?;
            Ok(regex.is_match(line))
        } else {
            // Default: if no explicit continue pattern, consider non-timestamp lines as continuation
            Ok(!self.has_timestamp_prefix(line))
        }
    }

    fn has_timestamp_prefix(&self, line: &str) -> bool {
        // Common timestamp patterns
        let patterns = [
            r"^\d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2}",  // ISO format
            r"^\d{2}/\d{2}/\d{4}\s+\d{2}:\d{2}:\d{2}",     // US format
            r"^\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}",       // Syslog format
            r"^\[\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\]", // Bracketed format
        ];

        for pattern in &patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(line) {
                    return true;
                }
            }
        }
        false
    }

    fn flush_multiline_buffer(
        &mut self,
        line_number: u64,
        byte_offset: u64,
    ) -> Result<Option<LogEntry>, ParserError> {
        if self.multiline_buffer.is_empty() {
            return Ok(None);
        }

        let content = self.multiline_buffer.join("\n");
        let entry = LogEntry {
            content,
            line_number: line_number - self.multiline_buffer.len() as u64,
            byte_offset,
            is_multiline: true,
            multiline_group_id: Some(self.multiline_group_counter),
        };

        self.multiline_buffer.clear();
        Ok(Some(entry))
    }

    fn parse_single_entry(&self, entry: &LogEntry) -> Result<ParsedLogLine, ParserError> {
        // Use format-specific parser
        let mut parsed = self.format_specific_parser.parse(&entry.content)?;
        
        // Fill in metadata
        parsed.raw_content = entry.content.clone();
        parsed.line_number = entry.line_number;
        parsed.byte_offset = entry.byte_offset;
        parsed.is_multiline = entry.is_multiline;
        parsed.multiline_group_id = entry.multiline_group_id;

        Ok(parsed)
    }

    fn create_format_parser(format: &LogFormat) -> Result<Box<dyn LogFormatParser + Send + Sync>, ParserError> {
        match format {
            LogFormat::Plain => Ok(Box::new(PlainTextParser::new())),
            LogFormat::JsonLines => Ok(Box::new(JsonLinesParser::new())),
            LogFormat::CustomRegex(pattern) => Ok(Box::new(CustomRegexParser::new(pattern.clone())?)),
            LogFormat::CommonLogFormat => Ok(Box::new(ApacheCommonLogParser::new())),
            LogFormat::Combined => Ok(Box::new(ApacheCombinedLogParser::new())),
            LogFormat::Nginx => Ok(Box::new(NginxLogParser::new())),
            LogFormat::Syslog => Ok(Box::new(SyslogParser::new())),
        }
    }
}

// Plain Text Parser
pub struct PlainTextParser {
    log_level_regex: Regex,
    timestamp_regex: Regex,
}

impl PlainTextParser {
    pub fn new() -> Self {
        Self {
            log_level_regex: Regex::new(r"\b(TRACE|DEBUG|INFO|WARN|ERROR|FATAL)\b").unwrap(),
            timestamp_regex: Regex::new(r"(\d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2}(?:\.\d{3})?(?:Z|[+-]\d{2}:\d{2})?)|(\d{2}/\d{2}/\d{4}\s+\d{2}:\d{2}:\d{2})|(\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})").unwrap(),
        }
    }
}

impl LogFormatParser for PlainTextParser {
    fn parse(&self, content: &str) -> Result<ParsedLogLine, ParserError> {
        let fields = HashMap::new();
        
        // Extract log level
        let level = if let Some(captures) = self.log_level_regex.captures(content) {
            let level_str = captures.get(0).unwrap().as_str();
            Some(match level_str {
                "TRACE" => LogLevel::Trace,
                "DEBUG" => LogLevel::Debug,
                "INFO" => LogLevel::Info,
                "WARN" => LogLevel::Warn,
                "ERROR" => LogLevel::Error,
                "FATAL" => LogLevel::Fatal,
                _ => LogLevel::Info,
            })
        } else {
            None
        };

        // Extract timestamp
        let timestamp = if let Some(captures) = self.timestamp_regex.captures(content) {
            let timestamp_str = captures.get(0).unwrap().as_str();
            Some(timestamp_str.to_string())
        } else {
            None
        };

        // Message is the full content for plain text
        let message = content.to_string();

        Ok(ParsedLogLine {
            raw_content: content.to_string(),
            line_number: 0, // Will be set by caller
            byte_offset: 0, // Will be set by caller
            timestamp,
            level,
            message,
            fields,
            is_multiline: false, // Will be set by caller
            multiline_group_id: None, // Will be set by caller
        })
    }
}

impl PlainTextParser {
    fn normalize_timestamp(&self, timestamp_str: &str) -> String {
        // Return normalized timestamp string - could be enhanced with proper parsing
        timestamp_str.to_string()
    }
}

// JSON Lines Parser
pub struct JsonLinesParser;

impl JsonLinesParser {
    pub fn new() -> Self {
        Self
    }
}

impl LogFormatParser for JsonLinesParser {
    fn parse(&self, content: &str) -> Result<ParsedLogLine, ParserError> {
        let json_value: Value = serde_json::from_str(content.trim())?;
        let mut fields = HashMap::new();

        // Extract common fields
        let timestamp = if let Some(ts_value) = json_value.get("timestamp")
            .or(json_value.get("@timestamp"))
            .or(json_value.get("time"))
        {
            if let Some(ts_str) = ts_value.as_str() {
                Some(ts_str.to_string())
            } else {
                None
            }
        } else {
            None
        };

        let level = if let Some(level_value) = json_value.get("level")
            .or(json_value.get("severity"))
            .or(json_value.get("logLevel"))
        {
            if let Some(level_str) = level_value.as_str() {
                Some(match level_str.to_uppercase().as_str() {
                    "TRACE" => LogLevel::Trace,
                    "DEBUG" => LogLevel::Debug,
                    "INFO" => LogLevel::Info,
                    "WARN" | "WARNING" => LogLevel::Warn,
                    "ERROR" => LogLevel::Error,
                    "FATAL" => LogLevel::Fatal,
                    _ => LogLevel::Info,
                })
            } else {
                None
            }
        } else {
            None
        };

        let message = json_value.get("message")
            .or(json_value.get("msg"))
            .and_then(|v| v.as_str())
            .unwrap_or(content)
            .to_string();

        // Store all JSON fields
        if let Value::Object(map) = json_value {
            for (key, value) in map {
                fields.insert(key, value);
            }
        }

        Ok(ParsedLogLine {
            raw_content: content.to_string(),
            line_number: 0,
            byte_offset: 0,
            timestamp,
            level,
            message,
            fields,
            is_multiline: false,
            multiline_group_id: None,
        })
    }
}

// Placeholder implementations for other parsers
pub struct CustomRegexParser {
    regex: Regex,
}

impl CustomRegexParser {
    pub fn new(pattern: String) -> Result<Self, ParserError> {
        let regex = Regex::new(&pattern)?;
        Ok(Self { regex })
    }
}

impl LogFormatParser for CustomRegexParser {
    fn parse(&self, content: &str) -> Result<ParsedLogLine, ParserError> {
        // Basic implementation - in a full version this would use named capture groups
        Ok(ParsedLogLine {
            raw_content: content.to_string(),
            line_number: 0,
            byte_offset: 0,
            timestamp: None,
            level: None,
            message: content.to_string(),
            fields: HashMap::new(),
            is_multiline: false,
            multiline_group_id: None,
        })
    }
}

// Placeholder parsers for other formats
pub struct ApacheCommonLogParser;
impl ApacheCommonLogParser { pub fn new() -> Self { Self } }
impl LogFormatParser for ApacheCommonLogParser {
    fn parse(&self, content: &str) -> Result<ParsedLogLine, ParserError> {
        Ok(ParsedLogLine {
            raw_content: content.to_string(), line_number: 0, byte_offset: 0,
            timestamp: None, level: None, message: content.to_string(),
            fields: HashMap::new(), is_multiline: false, multiline_group_id: None,
        })
    }
}

pub struct ApacheCombinedLogParser;
impl ApacheCombinedLogParser { pub fn new() -> Self { Self } }
impl LogFormatParser for ApacheCombinedLogParser {
    fn parse(&self, content: &str) -> Result<ParsedLogLine, ParserError> {
        Ok(ParsedLogLine {
            raw_content: content.to_string(), line_number: 0, byte_offset: 0,
            timestamp: None, level: None, message: content.to_string(),
            fields: HashMap::new(), is_multiline: false, multiline_group_id: None,
        })
    }
}

pub struct NginxLogParser;
impl NginxLogParser { pub fn new() -> Self { Self } }
impl LogFormatParser for NginxLogParser {
    fn parse(&self, content: &str) -> Result<ParsedLogLine, ParserError> {
        Ok(ParsedLogLine {
            raw_content: content.to_string(), line_number: 0, byte_offset: 0,
            timestamp: None, level: None, message: content.to_string(),
            fields: HashMap::new(), is_multiline: false, multiline_group_id: None,
        })
    }
}

pub struct SyslogParser;
impl SyslogParser { pub fn new() -> Self { Self } }
impl LogFormatParser for SyslogParser {
    fn parse(&self, content: &str) -> Result<ParsedLogLine, ParserError> {
        Ok(ParsedLogLine {
            raw_content: content.to_string(), line_number: 0, byte_offset: 0,
            timestamp: None, level: None, message: content.to_string(),
            fields: HashMap::new(), is_multiline: false, multiline_group_id: None,
        })
    }
}