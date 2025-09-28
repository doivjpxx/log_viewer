// File types from Rust backend
export interface FileInfo {
  id?: string;
  path: string;
  name: string;
  size: number;
  encoding: string;
  lines: number;
  lastModified: string;
}

export interface FileChunk {
  startLine: number;
  endLine: number;
  content: string[];
  totalLines?: number;
}

export interface SearchOptions {
  query: string;
  is_regex: boolean;
  is_case_sensitive: boolean;
  is_whole_word: boolean;
  max_results: number;
  context_lines?: number;
  start_line?: number;
  end_line?: number;
}

export interface SearchResult {
  line_number: number;
  content: string;
  highlights: HighlightRange[];
  context_before?: string[];
  context_after?: string[];
}

export interface HighlightRange {
  start: number;
  end: number;
}

// Parser types
export type LogFormat = 
  | 'Plain'
  | 'JsonLines'
  | { CustomRegex: string }
  | 'CommonLogFormat'
  | 'Combined'
  | 'Nginx'
  | 'Syslog';

export interface MultilineConfig {
  enabled: boolean;
  start_pattern?: string;
  continue_pattern?: string;
  end_pattern?: string;
  max_lines: number;
}

export interface FieldExtractor {
  name: string;
  regex: string;
  data_type: FieldType;
}

export type FieldType = 'String' | 'Number' | 'Boolean' | 'Timestamp';

export interface ParsedLogLine {
  raw_content: string;
  line_number: number;
  byte_offset: number;
  timestamp?: string;
  level?: LogLevel;
  message: string;
  fields: Record<string, unknown>;
  is_multiline: boolean;
  multiline_group_id?: number;
}

export type LogLevel = 'Trace' | 'Debug' | 'Info' | 'Warn' | 'Error' | 'Fatal';

export interface ParserConfig {
  format: LogFormat;
  multiline: MultilineConfig;
  timestamp_format?: string;
  custom_fields: FieldExtractor[];
}

// UI types
export type ViewMode = 'list' | 'table';

export interface TableColumn {
  field: string;
  title: string;
  width?: number;
  minWidth?: number;
  type?: string;
  formatter?: (value: unknown, row: Record<string, unknown>) => string;
  sortable?: boolean;
  resizable?: boolean;
}

// Log line for table view
export interface TableLogLine {
  id: number;
  lineNumber: number;
  timestamp?: string;
  level?: LogLevel;
  message: string;
  rawContent: string;
  [key: string]: unknown; // For dynamic parsed fields
}