import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Types for parser functionality - matching Rust types
export type LogFormat = 
  | 'Plain'
  | 'JsonLines'
  | { CustomRegex: string }
  | 'CommonLogFormat'
  | 'Combined'
  | 'Nginx'
  | 'Syslog'

export interface ParserConfig {
  format: LogFormat
  multiline: MultilineConfig
  timestamp_format?: string
  custom_fields: FieldExtractor[]
}

export interface MultilineConfig {
  enabled: boolean
  start_pattern?: string
  continue_pattern?: string
  end_pattern?: string
  max_lines: number
}

export interface FieldExtractor {
  name: string
  regex: string
  data_type: 'String' | 'Number' | 'Boolean' | 'Timestamp'
}

export interface FormatDetectionResult {
  detected_format: LogFormat
  confidence: number
  sample_parsed_lines: ParsedLogLine[]
  suggested_multiline_config?: MultilineConfig
}

export interface ParsedLogLine {
  raw_content: string
  line_number: number
  byte_offset: number
  timestamp?: string
  level?: 'Trace' | 'Debug' | 'Info' | 'Warn' | 'Error' | 'Fatal'
  message: string
  fields: Record<string, unknown>
  is_multiline: boolean
  multiline_group_id?: number
}

export const useParserStore = defineStore('parser', () => {
  // State
  const currentConfig = ref<ParserConfig>({
    format: 'Plain',
    multiline: {
      enabled: false,
      max_lines: 50
    },
    custom_fields: []
  })
  
  const detectedFormat = ref<FormatDetectionResult | null>(null)
  const isAutoDetectionEnabled = ref(true)
  const parsingEnabled = ref(false)
  const parsedLines = ref<ParsedLogLine[]>([])
  
  // Loading states
  const isDetecting = ref(false)
  const isParsing = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const formatName = computed(() => {
    const format = currentConfig.value.format
    if (format === 'Plain') return 'Plain Text'
    if (format === 'JsonLines') return 'JSON Lines'
    if (typeof format === 'object' && 'CustomRegex' in format) return 'Custom Regex'
    if (format === 'CommonLogFormat') return 'Apache Common Log'
    if (format === 'Combined') return 'Apache Combined Log'
    if (format === 'Nginx') return 'Nginx'
    if (format === 'Syslog') return 'Syslog'
    return 'Unknown'
  })

  const isMultilineEnabled = computed(() => currentConfig.value.multiline.enabled)
  
  const hasStructuredData = computed(() => {
    return parsedLines.value.some(line => Object.keys(line.fields).length > 0)
  })

  // Actions
  const detectFormat = async (filePath: string, sampleSize: number = 20): Promise<void> => {
    try {
      isDetecting.value = true
      error.value = null
      
      const result: FormatDetectionResult = await invoke('detect_log_format', {
        path: filePath,
        sampleSize
      })
      
      detectedFormat.value = result
      
      // Auto-apply detected format if auto-detection is enabled
      if (isAutoDetectionEnabled.value) {
        currentConfig.value.format = result.detected_format
        
        // Apply suggested multiline config
        if (result.suggested_multiline_config) {
          currentConfig.value.multiline = result.suggested_multiline_config
        }
      }
      
    } catch (err) {
      // Format detection failed
      error.value = `Format detection failed: ${err}`
    } finally {
      isDetecting.value = false
    }
  }

  const parseLines = async (
    lines: string[], 
    startLineNumber: number = 0, 
    startByteOffset: number = 0
  ): Promise<ParsedLogLine[]> => {
    try {
      isParsing.value = true
      error.value = null
      
      const result: ParsedLogLine[] = await invoke('parse_lines_with_config', {
        lines,
        config: currentConfig.value,
        startLineNumber,
        startByteOffset
      })
      
      parsedLines.value = result
      return result
      
    } catch (err) {
      // Line parsing failed
      error.value = `Line parsing failed: ${err}`
      return []
    } finally {
      isParsing.value = false
    }
  }

  const getParsedLinesRange = async (
    filePath: string,
    startLine: number,
    endLine: number
  ): Promise<ParsedLogLine[]> => {
    try {
      isParsing.value = true
      error.value = null
      
      const result: ParsedLogLine[] = await invoke('get_parsed_lines_range', {
        path: filePath,
        startLine,
        endLine,
        config: currentConfig.value
      })
      
      return result
      
    } catch (err) {
      // Parsed lines range retrieval failed
      error.value = `Parsed lines range retrieval failed: ${err}`
      return []
    } finally {
      isParsing.value = false
    }
  }

  const setFormat = (format: LogFormat): void => {
    currentConfig.value.format = format
    parsingEnabled.value = true
  }

  const setMultilineConfig = (config: MultilineConfig): void => {
    currentConfig.value.multiline = config
  }

  const addCustomField = (field: FieldExtractor): void => {
    currentConfig.value.custom_fields.push(field)
  }

  const removeCustomField = (index: number): void => {
    currentConfig.value.custom_fields.splice(index, 1)
  }

  const toggleParsing = (): void => {
    parsingEnabled.value = !parsingEnabled.value
  }

  const toggleAutoDetection = (): void => {
    isAutoDetectionEnabled.value = !isAutoDetectionEnabled.value
  }

  const resetConfig = (): void => {
    currentConfig.value = {
      format: 'Plain',
      multiline: {
        enabled: false,
        max_lines: 50
      },
      custom_fields: []
    }
    detectedFormat.value = null
    parsedLines.value = []
    parsingEnabled.value = false
    error.value = null
  }

  // Preset configurations
  const applyPresetConfig = (preset: string): void => {
    switch (preset) {
      case 'json':
        setFormat('JsonLines')
        setMultilineConfig({
          enabled: true,
          start_pattern: '^\\s*\\{',
          continue_pattern: '^\\s+',
          end_pattern: '^\\s*\\}',
          max_lines: 100
        })
        break
      
      case 'java-stack-trace':
        setFormat('Plain')
        setMultilineConfig({
          enabled: true,
          start_pattern: '^\\d{4}-\\d{2}-\\d{2}|^\\w{3}\\s+\\d{1,2}',
          continue_pattern: '^\\s+at\\s+|^\\s+\\.\\.\\.|^\\s*Caused by:',
          max_lines: 50
        })
        break
      
      case 'apache-combined':
        setFormat('Combined')
        setMultilineConfig({ enabled: false, max_lines: 1 })
        break
      
      case 'syslog':
        setFormat('Syslog')
        setMultilineConfig({ enabled: false, max_lines: 1 })
        break
      
      default:
        resetConfig()
    }
  }

  return {
    // State
    currentConfig,
    detectedFormat,
    isAutoDetectionEnabled,
    parsingEnabled,
    parsedLines,
    isDetecting,
    isParsing,
    error,
    
    // Getters
    formatName,
    isMultilineEnabled,
    hasStructuredData,
    
    // Actions
    detectFormat,
    parseLines,
    getParsedLinesRange,
    setFormat,
    setMultilineConfig,
    addCustomField,
    removeCustomField,
    toggleParsing,
    toggleAutoDetection,
    resetConfig,
    applyPresetConfig
  }
})