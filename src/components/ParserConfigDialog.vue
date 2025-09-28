<template>
  <TransitionRoot :show="uiStore.isParserConfigOpen" as="template">
    <Dialog class="relative z-50" @close="uiStore.toggleParserConfig">
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
            <DialogPanel class="parser-dialog">
              <div class="parser-dialog-content">
                <!-- Header -->
                <div class="dialog-header">
                  <DialogTitle class="dialog-title">
                    <span class="dialog-icon">⚙️</span>
                    Parser Configuration
                  </DialogTitle>
                  <button @click="uiStore.toggleParserConfig" class="dialog-close">
                    ✕
                  </button>
                </div>

                <!-- Content -->
                <div class="dialog-body">
                  <!-- Format Detection -->
                  <div class="config-section">
                    <h3 class="section-title">Format Detection</h3>
                    
                    <!-- Auto Detection Toggle -->
                    <div class="config-item">
                      <label class="toggle-label">
                        <input
                          type="checkbox"
                          v-model="parserStore.isAutoDetectionEnabled"
                          class="toggle-input"
                        />
                        <span class="toggle-text">Auto-detect log format</span>
                      </label>
                    </div>

                    <!-- Manual Format Selection -->
                    <div class="config-item" v-if="!parserStore.isAutoDetectionEnabled">
                      <label class="config-label">Log Format</label>
                      <select v-model="selectedFormat" class="config-select">
                        <option value="Plain">Plain Text</option>
                        <option value="JsonLines">JSON Lines</option>
                        <option value="CommonLogFormat">Apache Common Log</option>
                        <option value="Combined">Apache Combined Log</option>
                        <option value="Nginx">Nginx</option>
                        <option value="Syslog">Syslog</option>
                        <option value="CustomRegex">Custom Regex</option>
                      </select>
                    </div>

                    <!-- Custom Regex Pattern -->
                    <div class="config-item" v-if="selectedFormat === 'CustomRegex'">
                      <label class="config-label">Regex Pattern</label>
                      <input
                        type="text"
                        v-model="customRegexPattern"
                        placeholder="Enter regex pattern..."
                        class="config-input"
                      />
                    </div>

                    <!-- Detection Results -->
                    <div v-if="parserStore.detectedFormat" class="detection-results">
                      <h4 class="results-title">Detection Results</h4>
                      <div class="detection-info">
                        <span class="detected-format">{{ getFormatName(parserStore.detectedFormat.detected_format) }}</span>
                        <span class="confidence" :class="getConfidenceClass(parserStore.detectedFormat.confidence)">
                          {{ Math.round(parserStore.detectedFormat.confidence * 100) }}% confidence
                        </span>
                      </div>
                    </div>
                  </div>

                  <!-- Multiline Configuration -->
                  <div class="config-section">
                    <h3 class="section-title">Multiline Processing</h3>
                    
                    <!-- Multiline Toggle -->
                    <div class="config-item">
                      <label class="toggle-label">
                        <input
                          type="checkbox"
                          v-model="multilineConfig.enabled"
                          class="toggle-input"
                        />
                        <span class="toggle-text">Enable multiline parsing</span>
                      </label>
                    </div>

                    <!-- Multiline Settings -->
                    <div v-if="multilineConfig.enabled" class="multiline-settings">
                      <div class="config-item">
                        <label class="config-label">Start Pattern (optional)</label>
                        <input
                          type="text"
                          v-model="multilineConfig.start_pattern"
                          placeholder="Regex pattern for line start"
                          class="config-input"
                        />
                      </div>

                      <div class="config-item">
                        <label class="config-label">Continue Pattern (optional)</label>
                        <input
                          type="text"
                          v-model="multilineConfig.continue_pattern"
                          placeholder="Regex pattern for continuation lines"
                          class="config-input"
                        />
                      </div>

                      <div class="config-item">
                        <label class="config-label">Max Lines per Entry</label>
                        <input
                          type="number"
                          v-model="multilineConfig.max_lines"
                          min="1"
                          max="1000"
                          class="config-input"
                        />
                      </div>
                    </div>
                  </div>

                  <!-- Preset Configurations -->
                  <div class="config-section">
                    <h3 class="section-title">Quick Presets</h3>
                    <div class="preset-buttons">
                      <button
                        v-for="preset in presets"
                        :key="preset.id"
                        @click="applyPreset(preset.id)"
                        class="preset-button"
                      >
                        <span class="preset-icon">{{ preset.icon }}</span>
                        <span class="preset-name">{{ preset.name }}</span>
                      </button>
                    </div>
                  </div>

                  <!-- Status and Errors -->
                  <div v-if="parserStore.error" class="error-section">
                    <div class="error-message">
                      <span class="error-icon">⚠️</span>
                      {{ parserStore.error }}
                    </div>
                  </div>
                </div>

                <!-- Footer -->
                <div class="dialog-footer">
                  <button
                    @click="detectFormat"
                    :disabled="parserStore.isDetecting || !currentFilePath"
                    class="btn btn-secondary"
                  >
                    <span v-if="parserStore.isDetecting" class="spinner"></span>
                    {{ parserStore.isDetecting ? 'Detecting...' : 'Detect Format' }}
                  </button>
                  <button @click="applyConfiguration" class="btn btn-primary">
                    Apply Configuration
                  </button>
                  <button @click="uiStore.toggleParserConfig" class="btn btn-ghost">
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
import { ref, computed, watch } from 'vue'
import { storeToRefs } from 'pinia'
import {
  Dialog,
  DialogPanel,
  DialogTitle,
  TransitionChild,
  TransitionRoot,
} from '@headlessui/vue'
import { useUIStore } from '../stores/uiStore'
import { useParserStore } from '../stores/parserStore'
import { useFileStore } from '../stores/fileStore'
import type { LogFormat, MultilineConfig } from '../stores/parserStore'

const uiStore = useUIStore()
const parserStore = useParserStore()
const fileStore = useFileStore()

// Local reactive state
const selectedFormat = ref<LogFormat>('Plain')
const customRegexPattern = ref('')
const multilineConfig = ref<MultilineConfig>({
  enabled: false,
  max_lines: 50
})

// Computed
const currentFilePath = computed(() => fileStore.currentFile?.path)

// Presets
const presets = [
  { id: 'json', name: 'JSON Lines', icon: '📄' },
  { id: 'java-stack-trace', name: 'Java Stack Traces', icon: '🔥' },
  { id: 'apache-combined', name: 'Apache Combined', icon: '🌐' },
  { id: 'syslog', name: 'Syslog', icon: '📊' },
]

// Watch for changes from store
watch(() => parserStore.currentConfig, (config) => {
  selectedFormat.value = config.format
  multilineConfig.value = { ...config.multiline }
}, { deep: true, immediate: true })

// Update custom regex pattern when format changes
watch(selectedFormat, (format) => {
  if (typeof format === 'object' && 'CustomRegex' in format) {
    customRegexPattern.value = format.CustomRegex
  }
})

// Methods
const detectFormat = async (): Promise<void> => {
  if (!currentFilePath.value) return
  await parserStore.detectFormat(currentFilePath.value)
}

const applyPreset = (presetId: string): void => {
  parserStore.applyPresetConfig(presetId)
}

const applyConfiguration = (): void => {
  // Set format
  let format: LogFormat = selectedFormat.value
  if (selectedFormat.value === 'CustomRegex') {
    format = { CustomRegex: customRegexPattern.value }
  }
  
  parserStore.setFormat(format)
  parserStore.setMultilineConfig(multilineConfig.value)
  
  // Close dialog
  uiStore.toggleParserConfig()
}

const getFormatName = (format: LogFormat): string => {
  if (format === 'Plain') return 'Plain Text'
  if (format === 'JsonLines') return 'JSON Lines'
  if (typeof format === 'object' && 'CustomRegex' in format) return 'Custom Regex'
  if (format === 'CommonLogFormat') return 'Apache Common Log'
  if (format === 'Combined') return 'Apache Combined Log'
  if (format === 'Nginx') return 'Nginx'
  if (format === 'Syslog') return 'Syslog'
  return 'Unknown'
}

const getConfidenceClass = (confidence: number): string => {
  if (confidence >= 0.8) return 'confidence-high'
  if (confidence >= 0.5) return 'confidence-medium'
  return 'confidence-low'
}
</script>

<style scoped>
.parser-dialog {
  @apply relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all;
  @apply sm:my-8 sm:w-full sm:max-w-2xl;
  max-height: 80vh;
}

.parser-dialog-content {
  @apply flex flex-col;
  max-height: 80vh;
}

.dialog-header {
  @apply flex items-center justify-between px-6 py-4 border-b border-gray-200;
}

.dialog-title {
  @apply flex items-center text-lg font-semibold text-gray-900;
  gap: 8px;
}

.dialog-icon {
  @apply text-xl;
}

.dialog-close {
  @apply text-gray-400 hover:text-gray-600 text-xl font-semibold;
  @apply w-6 h-6 flex items-center justify-center;
}

.dialog-body {
  @apply flex-1 overflow-y-auto px-6 py-4 space-y-6;
}

.config-section {
  @apply space-y-4;
}

.section-title {
  @apply text-base font-semibold text-gray-900 border-b pb-2;
}

.config-item {
  @apply space-y-2;
}

.config-label {
  @apply block text-sm font-medium text-gray-700;
}

.config-input,
.config-select {
  @apply w-full px-3 py-2 border border-gray-300 rounded-md;
  @apply focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent;
}

.toggle-label {
  @apply flex items-center space-x-3 cursor-pointer;
}

.toggle-input {
  @apply w-4 h-4 text-blue-600 rounded focus:ring-blue-500;
}

.toggle-text {
  @apply text-sm text-gray-700;
}

.detection-results {
  @apply mt-4 p-4 bg-gray-50 rounded-lg;
}

.results-title {
  @apply text-sm font-medium text-gray-900 mb-2;
}

.detection-info {
  @apply flex items-center justify-between;
}

.detected-format {
  @apply font-semibold text-blue-600;
}

.confidence {
  @apply text-sm font-medium;
}

.confidence-high {
  @apply text-green-600;
}

.confidence-medium {
  @apply text-yellow-600;
}

.confidence-low {
  @apply text-red-600;
}

.multiline-settings {
  @apply ml-6 space-y-4 border-l-2 border-gray-200 pl-4;
}

.preset-buttons {
  @apply grid grid-cols-2 gap-3;
}

.preset-button {
  @apply flex items-center space-x-2 p-3 border border-gray-200 rounded-lg;
  @apply hover:bg-gray-50 hover:border-gray-300 transition-colors;
}

.preset-icon {
  @apply text-lg;
}

.preset-name {
  @apply text-sm font-medium text-gray-700;
}

.error-section {
  @apply mt-4;
}

.error-message {
  @apply flex items-center space-x-2 p-3 bg-red-50 border border-red-200 rounded-lg text-red-700;
}

.error-icon {
  @apply text-lg;
}

.dialog-footer {
  @apply flex justify-end space-x-3 px-6 py-4 border-t border-gray-200 bg-gray-50;
}

.btn {
  @apply px-4 py-2 text-sm font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2;
}

.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500;
}

.btn-secondary {
  @apply bg-gray-600 text-white hover:bg-gray-700 focus:ring-gray-500 disabled:opacity-50 disabled:cursor-not-allowed;
}

.btn-ghost {
  @apply bg-transparent text-gray-700 hover:bg-gray-100 focus:ring-gray-500;
}

.spinner {
  @apply inline-block w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin mr-2;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .parser-dialog {
    @apply bg-gray-800 text-white;
  }
  
  .dialog-header {
    @apply border-gray-700;
  }
  
  .dialog-title {
    @apply text-white;
  }
  
  .section-title {
    @apply text-white border-gray-700;
  }
  
  .config-label {
    @apply text-gray-300;
  }
  
  .config-input,
  .config-select {
    @apply bg-gray-700 border-gray-600 text-white;
  }
  
  .toggle-text {
    @apply text-gray-300;
  }
  
  .detection-results {
    @apply bg-gray-700;
  }
  
  .results-title {
    @apply text-white;
  }
  
  .multiline-settings {
    @apply border-gray-600;
  }
  
  .preset-button {
    @apply border-gray-600 hover:bg-gray-700;
  }
  
  .preset-name {
    @apply text-gray-300;
  }
  
  .dialog-footer {
    @apply border-gray-700 bg-gray-700;
  }
  
  .btn-ghost {
    @apply text-gray-300 hover:bg-gray-600;
  }
}
</style>