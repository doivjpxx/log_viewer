<template>
  <TransitionRoot :show="uiStore.isExportDialogOpen" as="template">
    <Dialog class="relative z-50" @close="uiStore.toggleExportDialog">
      <TransitionChild
        as="template"
        enter="ease-out duration-300"
        enter-from="opacity-0"
        enter-to="opacity-100"
        leave="ease-in duration-200"
        leave-from="opacity-100"
        leave-to="opacity-0"
      >
        <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
      </TransitionChild>

      <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
        <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
          <TransitionChild
            as="template"
            enter="ease-out duration-300"
            enter-from="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            enter-to="opacity-100 translate-y-0 sm:scale-100"
            leave="ease-in duration-200"
            leave-from="opacity-100 translate-y-0 sm:scale-100"
            leave-to="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
          >
            <DialogPanel class="export-dialog">
              <div class="export-dialog-content">
                <!-- Header -->
                <div class="dialog-header">
                  <DialogTitle class="dialog-title">
                    <span class="dialog-icon">📥</span>
                    Export Parsed Content
                  </DialogTitle>
                  <button @click="uiStore.toggleExportDialog" class="dialog-close">
                    ✕
                  </button>
                </div>

                <!-- Content -->
                <div class="dialog-body">
                  <!-- Export Format -->
                  <div class="config-section">
                    <h3 class="section-title">Export Format</h3>
                    
                    <div class="format-options">
                      <label class="format-option" :class="{ 'selected': exportOptions.format === 'json' }">
                        <input
                          type="radio"
                          value="json"
                          v-model="exportOptions.format"
                          class="radio-input"
                        />
                        <span class="format-label">JSON</span>
                        <span class="format-description">Structured JSON format with nested fields</span>
                      </label>

                      <label class="format-option" :class="{ 'selected': exportOptions.format === 'csv' }">
                        <input
                          type="radio"
                          value="csv"
                          v-model="exportOptions.format"
                          class="radio-input"
                        />
                        <span class="format-label">CSV</span>
                        <span class="format-description">Comma-separated values for spreadsheet import</span>
                      </label>

                      <label class="format-option" :class="{ 'selected': exportOptions.format === 'plain' }">
                        <input
                          type="radio"
                          value="plain"
                          v-model="exportOptions.format"
                          class="radio-input"
                        />
                        <span class="format-label">Plain Text</span>
                        <span class="format-description">Formatted plain text with timestamps</span>
                      </label>
                    </div>
                  </div>

                  <!-- Include Options -->
                  <div class="config-section">
                    <h3 class="section-title">Include Options</h3>
                    
                    <div class="include-options">
                      <label class="checkbox-option">
                        <input
                          type="checkbox"
                          v-model="exportOptions.include_raw_content"
                          class="checkbox-input"
                        />
                        <span class="checkbox-label">Raw Content</span>
                        <span class="checkbox-description">Original log line content</span>
                      </label>

                      <label class="checkbox-option">
                        <input
                          type="checkbox"
                          v-model="exportOptions.include_parsed_fields"
                          class="checkbox-input"
                        />
                        <span class="checkbox-label">Parsed Fields</span>
                        <span class="checkbox-description">Structured fields (timestamp, level, message)</span>
                      </label>

                      <label class="checkbox-option">
                        <input
                          type="checkbox"
                          v-model="exportOptions.include_metadata"
                          class="checkbox-input"
                        />
                        <span class="checkbox-label">Metadata</span>
                        <span class="checkbox-description">Line numbers, byte offsets, multiline info</span>
                      </label>
                    </div>
                  </div>

                  <!-- Range and Limits -->
                  <div class="config-section">
                    <h3 class="section-title">Export Range</h3>
                    
                    <div class="range-options">
                      <div class="input-group">
                        <label class="input-label">Start Line</label>
                        <input
                          type="number"
                          v-model="exportRange.startLine"
                          min="1"
                          class="range-input"
                          placeholder="1"
                        />
                      </div>

                      <div class="input-group">
                        <label class="input-label">End Line</label>
                        <input
                          type="number"
                          v-model="exportRange.endLine"
                          min="1"
                          class="range-input"
                          placeholder="All lines"
                        />
                      </div>

                      <div class="input-group">
                        <label class="input-label">Max Lines</label>
                        <input
                          type="number"
                          v-model="exportOptions.max_lines"
                          min="1"
                          max="100000"
                          class="range-input"
                          placeholder="No limit"
                        />
                      </div>
                    </div>
                  </div>

                  <!-- Performance Info -->
                  <div v-if="parserStore.performanceMetrics" class="config-section">
                    <h3 class="section-title">Performance Metrics</h3>
                    <div class="performance-info">
                      <div class="metric">
                        <span class="metric-label">Lines Processed:</span>
                        <span class="metric-value">{{ parserStore.performanceMetrics.lines_processed.toLocaleString() }}</span>
                      </div>
                      <div class="metric">
                        <span class="metric-label">Processing Speed:</span>
                        <span class="metric-value">{{ Math.round(parserStore.performanceMetrics.lines_per_second) }} lines/sec</span>
                      </div>
                      <div class="metric">
                        <span class="metric-label">Memory Usage:</span>
                        <span class="metric-value">{{ formatBytes(parserStore.performanceMetrics.memory_usage_bytes) }}</span>
                      </div>
                    </div>
                  </div>

                  <!-- Export Preview -->
                  <div v-if="exportPreview" class="config-section">
                    <h3 class="section-title">Export Preview</h3>
                    <div class="preview-container">
                      <pre class="export-preview">{{ exportPreview }}</pre>
                    </div>
                  </div>
                </div>

                <!-- Footer -->
                <div class="dialog-footer">
                  <button
                    @click="generatePreview"
                    :disabled="isGeneratingPreview"
                    class="button button-secondary"
                  >
                    {{ isGeneratingPreview ? 'Generating...' : 'Preview' }}
                  </button>
                  
                  <button
                    @click="exportContent"
                    :disabled="isExporting || !canExport"
                    class="button button-primary"
                  >
                    {{ isExporting ? 'Exporting...' : 'Export' }}
                  </button>
                  
                  <button
                    @click="uiStore.toggleExportDialog"
                    class="button button-ghost"
                  >
                    Cancel
                  </button>
                </div>
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue'
import { useUIStore } from '../stores/uiStore'
import { useParserStore, type ExportOptions } from '../stores/parserStore'
import { useFileStore } from '../stores/fileStore'

const uiStore = useUIStore()
const parserStore = useParserStore()
const fileStore = useFileStore()

// Export options
const exportOptions = reactive<ExportOptions>({
  format: 'json',
  include_raw_content: true,
  include_parsed_fields: true,
  include_metadata: false,
  max_lines: undefined
})

// Export range
const exportRange = reactive({
  startLine: 1,
  endLine: undefined as number | undefined
})

// UI state
const isExporting = ref(false)
const isGeneratingPreview = ref(false)
const exportPreview = ref('')

// Computed properties
const canExport = computed(() => {
  return fileStore.currentFile &&
    exportOptions.format &&
    (exportOptions.include_raw_content || exportOptions.include_parsed_fields || exportOptions.include_metadata)
})

// Methods
const generatePreview = async () => {
  if (!fileStore.currentFile) return

  try {
    isGeneratingPreview.value = true
    exportPreview.value = ''

    // Generate preview with limited lines
    const previewOptions = {
      ...exportOptions,
      max_lines: 5 // Limit preview to 5 lines
    }

    const result = await parserStore.exportParsedContent(
      fileStore.currentFile.path,
      exportRange.startLine,
      exportRange.endLine || exportRange.startLine + 10,
      previewOptions
    )

    exportPreview.value = result
  } catch (error) {
    console.error('Preview generation failed:', error)
    exportPreview.value = `Preview failed: ${error}`
  } finally {
    isGeneratingPreview.value = false
  }
}

const exportContent = async () => {
  if (!fileStore.currentFile) return

  try {
    isExporting.value = true
    
    const result = await parserStore.exportParsedContent(
      fileStore.currentFile.path,
      exportRange.startLine,
      exportRange.endLine || 1000000, // Large number for "all lines"
      exportOptions
    )

    // Create and download file
    const blob = new Blob([result], { type: getContentType() })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = getFilename()
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)

    // Close dialog
    uiStore.toggleExportDialog()
  } catch (error) {
    console.error('Export failed:', error)
  } finally {
    isExporting.value = false
  }
}

const getContentType = () => {
  switch (exportOptions.format) {
    case 'json':
      return 'application/json'
    case 'csv':
      return 'text/csv'
    case 'plain':
      return 'text/plain'
    default:
      return 'text/plain'
  }
}

const getFilename = () => {
  const baseName = fileStore.currentFile?.name.replace(/\.[^/.]+$/, '') || 'export'
  const timestamp = new Date().toISOString().slice(0, 19).replace(/:/g, '-')
  return `${baseName}_parsed_${timestamp}.${exportOptions.format === 'plain' ? 'txt' : exportOptions.format}`
}

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>

<style scoped>
.export-dialog {
  @apply relative transform rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-2xl;
}

.export-dialog-content {
  @apply w-full;
}

.dialog-header {
  @apply flex items-center justify-between border-b border-gray-200 px-6 py-4;
}

.dialog-title {
  @apply flex items-center gap-3 text-lg font-semibold text-gray-900;
}

.dialog-icon {
  @apply text-xl;
}

.dialog-close {
  @apply flex h-8 w-8 items-center justify-center rounded-full text-gray-400 hover:bg-gray-100 hover:text-gray-600;
}

.dialog-body {
  @apply max-h-96 overflow-y-auto px-6 py-4;
}

.config-section {
  @apply mb-6;
}

.section-title {
  @apply mb-3 text-sm font-medium text-gray-900;
}

.format-options {
  @apply space-y-2;
}

.format-option {
  @apply flex cursor-pointer items-start gap-3 rounded-lg border border-gray-200 p-3 transition-colors;
}

.format-option.selected {
  @apply border-blue-500 bg-blue-50;
}

.format-option:hover {
  @apply bg-gray-50;
}

.format-option.selected:hover {
  @apply bg-blue-50;
}

.radio-input {
  @apply mt-1 h-4 w-4 text-blue-600;
}

.format-label {
  @apply font-medium text-gray-900;
}

.format-description {
  @apply flex-1 text-sm text-gray-500;
}

.include-options {
  @apply space-y-3;
}

.checkbox-option {
  @apply flex cursor-pointer items-start gap-3;
}

.checkbox-input {
  @apply mt-1 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500;
}

.checkbox-label {
  @apply font-medium text-gray-900;
}

.checkbox-description {
  @apply flex-1 text-sm text-gray-500;
}

.range-options {
  @apply grid grid-cols-3 gap-4;
}

.input-group {
  @apply flex flex-col;
}

.input-label {
  @apply mb-1 text-sm font-medium text-gray-700;
}

.range-input {
  @apply block w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:ring-blue-500;
}

.performance-info {
  @apply grid grid-cols-1 gap-2 rounded-lg bg-gray-50 p-3 text-sm;
}

.metric {
  @apply flex justify-between;
}

.metric-label {
  @apply text-gray-600;
}

.metric-value {
  @apply font-medium text-gray-900;
}

.preview-container {
  @apply max-h-40 overflow-y-auto rounded-lg border border-gray-200;
}

.export-preview {
  @apply bg-gray-50 p-3 text-xs font-mono text-gray-800;
}

.dialog-footer {
  @apply flex justify-end gap-3 border-t border-gray-200 px-6 py-4;
}

.button {
  @apply inline-flex justify-center rounded-md px-4 py-2 text-sm font-medium focus:outline-none focus:ring-2 focus:ring-offset-2;
}

.button-primary {
  @apply border border-transparent bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500;
}

.button-secondary {
  @apply border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 focus:ring-blue-500;
}

.button-ghost {
  @apply border border-transparent text-gray-700 hover:bg-gray-50 focus:ring-gray-500;
}

.button:disabled {
  @apply cursor-not-allowed opacity-50;
}

/* Dark mode styles */
@media (prefers-color-scheme: dark) {
  .export-dialog {
    @apply bg-gray-800 text-white;
  }

  .dialog-header {
    @apply border-gray-700;
  }

  .dialog-title {
    @apply text-white;
  }

  .dialog-close {
    @apply text-gray-400 hover:bg-gray-700 hover:text-gray-200;
  }

  .section-title {
    @apply text-white;
  }

  .format-option {
    @apply border-gray-600 bg-gray-800;
  }

  .format-option.selected {
    @apply border-blue-400 bg-blue-900;
  }

  .format-option:hover {
    @apply bg-gray-700;
  }

  .format-label {
    @apply text-white;
  }

  .format-description {
    @apply text-gray-400;
  }

  .checkbox-label {
    @apply text-white;
  }

  .checkbox-description {
    @apply text-gray-400;
  }

  .input-label {
    @apply text-gray-300;
  }

  .range-input {
    @apply border-gray-600 bg-gray-700 text-white focus:border-blue-400 focus:ring-blue-400;
  }

  .performance-info {
    @apply bg-gray-700;
  }

  .metric-label {
    @apply text-gray-300;
  }

  .metric-value {
    @apply text-white;
  }

  .preview-container {
    @apply border-gray-600;
  }

  .export-preview {
    @apply bg-gray-700 text-gray-200;
  }

  .dialog-footer {
    @apply border-gray-700;
  }

  .button-secondary {
    @apply border-gray-600 bg-gray-700 text-gray-200 hover:bg-gray-600;
  }

  .button-ghost {
    @apply text-gray-300 hover:bg-gray-700;
  }
}
</style>