<template>
  <div 
    :class="lineClasses"
    :style="lineStyle"
    @click="handleClick"
    @dblclick="handleDoubleClick"
  >
    <!-- Line number -->
    <div class="line-number" :style="{ width: lineNumberWidth + 'px' }">
      {{ line.lineNumber + 1 }}
    </div>
    
    <!-- Line content -->
    <div class="line-content" v-html="formattedContent"></div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface LogLine {
  lineNumber: number
  content: string
  level?: 'INFO' | 'WARN' | 'ERROR' | 'DEBUG'
  timestamp?: string
}

interface Props {
  line: LogLine
  lineHeight: number
  isHighlighted?: boolean
  isSelected?: boolean
  searchHighlights?: Array<{ start: number; end: number }>
  lineNumberWidth?: number
}

const props = withDefaults(defineProps<Props>(), {
  isHighlighted: false,
  isSelected: false,
  searchHighlights: () => [],
  lineNumberWidth: 60,
})

const emit = defineEmits<{
  click: [line: LogLine, event: any]
  doubleClick: [line: LogLine, event: any]
}>()

// Computed
const lineClasses = computed(() => [
  'log-line',
  {
    'log-line--highlighted': props.isHighlighted,
    'log-line--selected': props.isSelected,
    'log-line--info': props.line.level === 'INFO',
    'log-line--warn': props.line.level === 'WARN',
    'log-line--error': props.line.level === 'ERROR',
    'log-line--debug': props.line.level === 'DEBUG',
  }
])

const lineStyle = computed(() => ({
  height: props.lineHeight + 'px',
  minHeight: props.lineHeight + 'px',
}))

const formattedContent = computed(() => {
  let content = escapeHtml(props.line.content)
  
  // Apply search highlights
  if (props.searchHighlights.length > 0) {
    content = applyHighlights(content, props.searchHighlights)
  }
  
  // Apply log level styling
  content = applyLogLevelStyling(content, props.line.level)
  
  return content
})

// Methods
function handleClick(event: any) {
  emit('click', props.line, event)
}

function handleDoubleClick(event: any) {
  emit('doubleClick', props.line, event)
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

function applyHighlights(content: string, highlights: Array<{ start: number; end: number }>): string {
  // Sort highlights by start position (descending) to apply from end to start
  const sortedHighlights = [...highlights].sort((a, b) => b.start - a.start)
  
  let result = content
  for (const highlight of sortedHighlights) {
    const before = result.substring(0, highlight.start)
    const highlighted = result.substring(highlight.start, highlight.end)
    const after = result.substring(highlight.end)
    result = before + `<mark class="search-highlight">${highlighted}</mark>` + after
  }
  
  return result
}

function applyLogLevelStyling(content: string, level?: string): string {
  if (!level) return content
  
  // Highlight log level keywords
  const levelPatterns = {
    'INFO': /(\[INFO\])/g,
    'WARN': /(\[WARN\])/g,
    'ERROR': /(\[ERROR\])/g,
    'DEBUG': /(\[DEBUG\])/g,
  }
  
  const pattern = levelPatterns[level as keyof typeof levelPatterns]
  if (pattern) {
    content = content.replace(pattern, `<span class="log-level log-level--${level.toLowerCase()}">$1</span>`)
  }
  
  // Highlight timestamps
  content = content.replace(
    /^(\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2})/,
    '<span class="timestamp">$1</span>'
  )
  
  return content
}
</script>

<style scoped>
.log-line {
  display: flex;
  align-items: flex-start;
  padding: 0 8px;
  border-bottom: 1px solid transparent;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 13px;
  line-height: 1.4;
  cursor: pointer;
  white-space: pre-wrap;
  word-break: break-word;
}

.log-line:hover {
  background-color: #f8f9fa;
}

.log-line--selected {
  background-color: #e3f2fd;
}

.log-line--highlighted {
  background-color: #fff3e0;
}

.log-line--info {
  border-left: 3px solid #2196f3;
}

.log-line--warn {
  border-left: 3px solid #ff9800;
}

.log-line--error {
  border-left: 3px solid #f44336;
}

.log-line--debug {
  border-left: 3px solid #9e9e9e;
}

.line-number {
  flex-shrink: 0;
  text-align: right;
  padding-right: 12px;
  color: #9e9e9e;
  user-select: none;
  font-size: 12px;
}

.line-content {
  flex: 1;
  min-width: 0;
}

/* Global styles for content formatting */
:deep(.search-highlight) {
  background-color: #ffeb3b;
  padding: 1px 2px;
  border-radius: 2px;
}

:deep(.log-level) {
  font-weight: 600;
  padding: 1px 4px;
  border-radius: 3px;
  font-size: 11px;
}

:deep(.log-level--info) {
  background-color: #e3f2fd;
  color: #1976d2;
}

:deep(.log-level--warn) {
  background-color: #fff3e0;
  color: #f57c00;
}

:deep(.log-level--error) {
  background-color: #ffebee;
  color: #d32f2f;
}

:deep(.log-level--debug) {
  background-color: #f5f5f5;
  color: #616161;
}

:deep(.timestamp) {
  color: #666;
  font-size: 12px;
}
</style>