import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { ViewMode, TableColumn, TableLogLine, ParsedLogLine, LogLevel } from '../types';

export const useTableStore = defineStore('table', () => {
  // State
  const viewMode = ref<ViewMode>('list');
  const tableData = ref<TableLogLine[]>([]);
  const selectedRows = ref<Set<number>>(new Set());
  const isLoading = ref(false);
  
  // Default columns for table view
  const defaultColumns = ref<TableColumn[]>([
    {
      field: 'lineNumber',
      title: 'Line',
      width: 80,
      type: 'seq',
      sortable: true
    },
    {
      field: 'timestamp',
      title: 'Timestamp',
      width: 180,
      sortable: true,
      formatter: (value: unknown) => {
        if (!value || typeof value !== 'string') return '-';
        try {
          return new Date(value).toLocaleString();
        } catch {
          return value;
        }
      }
    },
    {
      field: 'level',
      title: 'Level',
      width: 80,
      sortable: true,
      formatter: (value: unknown) => {
        if (!value || typeof value !== 'string') return '-';
        return value;
      }
    },
    {
      field: 'message',
      title: 'Message',
      minWidth: 300,
      sortable: false,
      formatter: (value: unknown) => {
        if (typeof value !== 'string') return String(value || '');
        return value;
      }
    }
  ]);
  
  const customColumns = ref<TableColumn[]>([]);
  
  // Computed
  const allColumns = computed(() => {
    return [...defaultColumns.value, ...customColumns.value];
  });
  
  const isTableView = computed(() => viewMode.value === 'table');
  const isListView = computed(() => viewMode.value === 'list');
  
  const selectedRowsCount = computed(() => selectedRows.value.size);
  
  // Actions
  function setViewMode(mode: ViewMode) {
    viewMode.value = mode;
  }
  
  function toggleViewMode() {
    viewMode.value = viewMode.value === 'list' ? 'table' : 'list';
  }
  
  function setTableData(data: TableLogLine[]) {
    tableData.value = data;
  }
  
  function addTableRow(row: TableLogLine) {
    tableData.value.push(row);
  }
  
  function clearTableData() {
    tableData.value = [];
    selectedRows.value.clear();
  }
  
  function selectRow(id: number) {
    selectedRows.value.add(id);
  }
  
  function deselectRow(id: number) {
    selectedRows.value.delete(id);
  }
  
  function toggleRowSelection(id: number) {
    if (selectedRows.value.has(id)) {
      selectedRows.value.delete(id);
    } else {
      selectedRows.value.add(id);
    }
  }
  
  function selectAllRows() {
    tableData.value.forEach(row => {
      selectedRows.value.add(row.id);
    });
  }
  
  function clearSelection() {
    selectedRows.value.clear();
  }
  
  function addCustomColumn(column: TableColumn) {
    customColumns.value.push(column);
  }
  
  function removeCustomColumn(field: string) {
    const index = customColumns.value.findIndex(col => col.field === field);
    if (index !== -1) {
      customColumns.value.splice(index, 1);
    }
  }
  
  function updateCustomColumns(columns: TableColumn[]) {
    customColumns.value = columns;
  }
  
  // Convert parsed log lines to table format
  function convertToTableFormat(lines: string[], startLineNumber = 0): TableLogLine[] {
    return lines.map((content, index) => {
      const lineNumber = startLineNumber + index + 1;
      const parsed = parseLogLine(content);
      
      return {
        id: startLineNumber + index,
        lineNumber,
        rawContent: content,
        message: parsed.message || content,
        timestamp: parsed.timestamp,
        level: parsed.level,
        ...parsed.fields
      };
    });
  }
  
  // Simple log line parser to extract common patterns
  function parseLogLine(line: string): {
    timestamp?: string;
    level?: LogLevel;
    message: string;
    fields: Record<string, unknown>;
  } {
    const result: {
      timestamp?: string;
      level?: LogLevel;
      message: string;
      fields: Record<string, unknown>;
    } = {
      message: line,
      fields: {}
    };
    
    // Try to parse timestamp (ISO format or common log formats)
    const timestampRegex = /^(\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d{3})?(?:Z|[+-]\d{2}:\d{2})?)/;
    const timestampMatch = line.match(timestampRegex);
    if (timestampMatch) {
      result.timestamp = timestampMatch[1];
      line = line.substring(timestampMatch[0].length).trim();
    }
    
    // Try to parse log level
    const levelRegex = /^(TRACE|DEBUG|INFO|WARN|WARNING|ERROR|FATAL|SEVERE)\b/i;
    const levelMatch = line.match(levelRegex);
    if (levelMatch) {
      const level = levelMatch[1].toUpperCase();
      // Map common log levels
      switch (level) {
        case 'WARNING':
          result.level = 'Warn';
          break;
        case 'SEVERE':
          result.level = 'Error';
          break;
        default:
          result.level = (level.charAt(0) + level.slice(1).toLowerCase()) as LogLevel;
      }
      line = line.substring(levelMatch[0].length).trim();
    }
    
    // The rest is the message
    result.message = line || result.message;
    
    // Try to extract key-value pairs from the message
    const kvRegex = /(\w+)[:=]\s*([^,\s]+)/g;
    let match;
    while ((match = kvRegex.exec(result.message)) !== null) {
      const key = match[1];
      let value: unknown = match[2];
      
      // Try to parse as number
      if (!isNaN(Number(value)) && value !== '') {
        value = Number(value);
      }
      
      result.fields[key] = value;
    }
    
    return result;
  }
  
  // Convert parsed log lines with structure to table format
  function convertParsedLinesToTable(parsedLines: ParsedLogLine[]): TableLogLine[] {
    return parsedLines.map(line => {
      const tableRow: TableLogLine = {
        id: line.line_number,
        lineNumber: line.line_number + 1,
        rawContent: line.raw_content,
        message: line.message,
        timestamp: line.timestamp,
        level: line.level
      };
      
      // Add parsed fields as additional columns
      Object.entries(line.fields).forEach(([key, value]) => {
        tableRow[key] = value;
      });
      
      return tableRow;
    });
  }
  
  // Auto-detect columns from parsed data
  function autoDetectColumns(data: TableLogLine[]) {
    if (data.length === 0) return;
    
    const sampleRow = data[0];
    const newCustomColumns: TableColumn[] = [];
    
    Object.keys(sampleRow).forEach(key => {
      // Skip default columns and metadata
      if (['id', 'lineNumber', 'timestamp', 'level', 'message', 'rawContent'].includes(key)) {
        return;
      }
      
      // Check if column already exists
      if (customColumns.value.some(col => col.field === key)) {
        return;
      }
      
      newCustomColumns.push({
        field: key,
        title: key.charAt(0).toUpperCase() + key.slice(1).replace(/_/g, ' '),
        width: 150,
        sortable: true,
        resizable: true
      });
    });
    
    customColumns.value.push(...newCustomColumns);
  }
  
  function getLogLevelColor(level?: LogLevel): string {
    switch (level) {
      case 'Error':
      case 'Fatal':
        return '#ef4444'; // red-500
      case 'Warn':
        return '#f59e0b'; // amber-500
      case 'Info':
        return '#3b82f6'; // blue-500
      case 'Debug':
        return '#6b7280'; // gray-500
      case 'Trace':
        return '#9ca3af'; // gray-400
      default:
        return '#374151'; // gray-700
    }
  }
  
  function setLoading(loading: boolean) {
    isLoading.value = loading;
  }
  
  return {
    // State
    viewMode,
    tableData,
    selectedRows,
    isLoading,
    defaultColumns,
    customColumns,
    
    // Computed
    allColumns,
    isTableView,
    isListView,
    selectedRowsCount,
    
    // Actions
    setViewMode,
    toggleViewMode,
    setTableData,
    addTableRow,
    clearTableData,
    selectRow,
    deselectRow,
    toggleRowSelection,
    selectAllRows,
    clearSelection,
    addCustomColumn,
    removeCustomColumn,
    updateCustomColumns,
    convertToTableFormat,
    convertParsedLinesToTable,
    autoDetectColumns,
    getLogLevelColor,
    setLoading
  };
});