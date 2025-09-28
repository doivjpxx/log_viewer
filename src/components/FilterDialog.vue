<template>
  <div v-if="showDialog" class="filter-dialog-overlay" @click="closeDialog">
    <div class="filter-dialog" :class="{ 'theme-dark': uiStore.isDarkTheme }" @click.stop>
      <div class="filter-header">
        <h3>Log Filters</h3>
        <button class="close-btn" @click="closeDialog">×</button>
      </div>

      <div class="filter-content">
        <!-- Log Level Filters -->
        <div class="filter-section">
          <h4>
            <span class="section-icon">📊</span>
            Log Levels
          </h4>
          <div class="log-level-filters">
            <label 
              v-for="level in logLevels" 
              :key="level.key"
              class="level-filter"
              :class="{ 
                active: activeFilters.logLevels.includes(level.key),
                'level-error': level.key === 'ERROR',
                'level-warn': level.key === 'WARN' || level.key === 'WARNING',
                'level-info': level.key === 'INFO',
                'level-debug': level.key === 'DEBUG' || level.key === 'TRACE'
              }"
            >
              <input
                v-model="activeFilters.logLevels"
                type="checkbox"
                :value="level.key"
                class="sr-only"
              />
              <div class="level-indicator" :style="{ backgroundColor: level.color }"></div>
              <span class="level-name">{{ level.name }}</span>
              <span class="level-count">{{ level.count }}</span>
            </label>
          </div>
          
          <div class="level-actions">
            <button class="action-btn" @click="selectAllLevels">Select All</button>
            <button class="action-btn" @click="selectNoneLevels">Select None</button>
            <button class="action-btn" @click="selectErrorsWarnings">Errors & Warnings</button>
          </div>
        </div>

        <!-- Text Filters -->
        <div class="filter-section">
          <h4>
            <span class="section-icon">🔍</span>
            Text Filters
          </h4>
          
          <!-- Include Filters -->
          <div class="text-filter-group">
            <label class="filter-label">Include lines containing:</label>
            <div class="filter-input-list">
              <div
                v-for="(filter, index) in activeFilters.includeText"
                :key="`include-${index}`"
                class="filter-input-item"
              >
                <input
                  v-model="filter.pattern"
                  type="text"
                  placeholder="Enter pattern..."
                  class="filter-input"
                  @keydown.enter="addIncludeFilter"
                />
                <select v-model="filter.type" class="filter-type-select">
                  <option value="contains">Contains</option>
                  <option value="regex">Regex</option>
                  <option value="exact">Exact</option>
                </select>
                <button class="remove-filter-btn" @click="removeIncludeFilter(index)">×</button>
              </div>
              <button class="add-filter-btn" @click="addIncludeFilter">+ Add Include Filter</button>
            </div>
          </div>
          
          <!-- Exclude Filters -->
          <div class="text-filter-group">
            <label class="filter-label">Exclude lines containing:</label>
            <div class="filter-input-list">
              <div
                v-for="(filter, index) in activeFilters.excludeText"
                :key="`exclude-${index}`"
                class="filter-input-item"
              >
                <input
                  v-model="filter.pattern"
                  type="text"
                  placeholder="Enter pattern..."
                  class="filter-input"
                  @keydown.enter="addExcludeFilter"
                />
                <select v-model="filter.type" class="filter-type-select">
                  <option value="contains">Contains</option>
                  <option value="regex">Regex</option>
                  <option value="exact">Exact</option>
                </select>
                <button class="remove-filter-btn" @click="removeExcludeFilter(index)">×</button>
              </div>
              <button class="add-filter-btn" @click="addExcludeFilter">+ Add Exclude Filter</button>
            </div>
          </div>
        </div>

        <!-- Time Range Filters -->
        <div class="filter-section">
          <h4>
            <span class="section-icon">⏰</span>
            Time Range
          </h4>
          <div class="time-filter-grid">
            <div class="time-input-group">
              <label for="start-time">Start Time</label>
              <input
                id="start-time"
                v-model="activeFilters.timeRange.start"
                type="datetime-local"
                class="time-input"
              />
            </div>
            <div class="time-input-group">
              <label for="end-time">End Time</label>
              <input
                id="end-time"
                v-model="activeFilters.timeRange.end"
                type="datetime-local"
                class="time-input"
              />
            </div>
          </div>
          
          <div class="time-presets">
            <button class="preset-btn" @click="setTimePreset('1h')">Last Hour</button>
            <button class="preset-btn" @click="setTimePreset('24h')">Last 24 Hours</button>
            <button class="preset-btn" @click="setTimePreset('7d')">Last Week</button>
            <button class="preset-btn" @click="clearTimeFilter">Clear</button>
          </div>
        </div>

        <!-- Filter Statistics -->
        <div v-if="filterStats" class="filter-stats">
          <div class="stat-item">
            <span class="stat-label">Total Lines:</span>
            <span class="stat-value">{{ formatNumber(filterStats.totalLines) }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Filtered Lines:</span>
            <span class="stat-value">{{ formatNumber(filterStats.filteredLines) }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Match Percentage:</span>
            <span class="stat-value">{{ filterStats.matchPercentage }}%</span>
          </div>
        </div>
      </div>

      <div class="filter-footer">
        <div class="filter-actions">
          <button class="btn btn-secondary" @click="resetFilters">
            Reset All
          </button>
          <button class="btn btn-secondary" @click="saveFilterPreset">
            Save Preset
          </button>
          <button
            class="btn btn-primary"
            :disabled="!hasActiveFilters"
            @click="applyFilters"
          >
            Apply Filters
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useFilterStore } from '../stores/filterStore'
import { useFileStore } from '../stores/fileStore'
import { useUIStore } from '../stores/uiStore'

const filterStore = useFilterStore()
const fileStore = useFileStore()
const uiStore = useUIStore()

interface TextFilter {
  pattern: string
  type: 'contains' | 'regex' | 'exact'
}

interface FilterStats {
  totalLines: number
  filteredLines: number
  matchPercentage: number
}

// State
const activeFilters = ref({
  logLevels: [] as string[],
  includeText: [] as TextFilter[],
  excludeText: [] as TextFilter[],
  timeRange: {
    start: '',
    end: ''
  }
})

const filterStats = ref<FilterStats | null>(null)

// Computed
const showDialog = computed(() => uiStore.showFilterDialog)

const logLevels = computed(() => [
  { key: 'ERROR', name: 'Error', color: '#ef4444', count: 0 },
  { key: 'WARN', name: 'Warning', color: '#f59e0b', count: 0 },
  { key: 'WARNING', name: 'Warning', color: '#f59e0b', count: 0 },
  { key: 'INFO', name: 'Info', color: '#3b82f6', count: 0 },
  { key: 'DEBUG', name: 'Debug', color: '#6b7280', count: 0 },
  { key: 'TRACE', name: 'Trace', color: '#9ca3af', count: 0 }
])

const hasActiveFilters = computed(() => {
  return (
    activeFilters.value.logLevels.length > 0 ||
    activeFilters.value.includeText.some(f => f.pattern.trim()) ||
    activeFilters.value.excludeText.some(f => f.pattern.trim()) ||
    activeFilters.value.timeRange.start ||
    activeFilters.value.timeRange.end
  )
})

// Methods
const closeDialog = () => {
  uiStore.toggleFilterDialog()
}

const selectAllLevels = () => {
  activeFilters.value.logLevels = logLevels.value.map(l => l.key)
}

const selectNoneLevels = () => {
  activeFilters.value.logLevels = []
}

const selectErrorsWarnings = () => {
  activeFilters.value.logLevels = ['ERROR', 'WARN', 'WARNING']
}

const addIncludeFilter = () => {
  activeFilters.value.includeText.push({
    pattern: '',
    type: 'contains'
  })
}

const removeIncludeFilter = (index: number) => {
  activeFilters.value.includeText.splice(index, 1)
}

const addExcludeFilter = () => {
  activeFilters.value.excludeText.push({
    pattern: '',
    type: 'contains'
  })
}

const removeExcludeFilter = (index: number) => {
  activeFilters.value.excludeText.splice(index, 1)
}

const setTimePreset = (preset: string) => {
  const now = new Date()
  const start = new Date()
  
  switch (preset) {
    case '1h':
      start.setHours(now.getHours() - 1)
      break
    case '24h':
      start.setDate(now.getDate() - 1)
      break
    case '7d':
      start.setDate(now.getDate() - 7)
      break
  }
  
  activeFilters.value.timeRange.start = start.toISOString().slice(0, 16)
  activeFilters.value.timeRange.end = now.toISOString().slice(0, 16)
}

const clearTimeFilter = () => {
  activeFilters.value.timeRange.start = ''
  activeFilters.value.timeRange.end = ''
}

const resetFilters = () => {
  activeFilters.value = {
    logLevels: [],
    includeText: [],
    excludeText: [],
    timeRange: { start: '', end: '' }
  }
  filterStats.value = null
}

const saveFilterPreset = () => {
  // TODO: Implement filter preset saving
  // Debug log for now
}

const applyFilters = async () => {
  try {
    // Convert filters to format expected by store/backend
    const filters = {
      logLevels: activeFilters.value.logLevels,
      includePatterns: activeFilters.value.includeText
        .filter(f => f.pattern.trim())
        .map(f => ({ pattern: f.pattern, type: f.type })),
      excludePatterns: activeFilters.value.excludeText
        .filter(f => f.pattern.trim())
        .map(f => ({ pattern: f.pattern, type: f.type })),
      timeRange: activeFilters.value.timeRange.start || activeFilters.value.timeRange.end 
        ? activeFilters.value.timeRange 
        : null
    }

    await filterStore.applyFilters(filters)
    
    // Update statistics
    filterStats.value = {
      totalLines: fileStore.currentFileLines,
      filteredLines: filterStore.filteredLineCount,
      matchPercentage: Math.round((filterStore.filteredLineCount / fileStore.currentFileLines) * 100)
    }
    
    closeDialog()
  } catch {
    // Error handling without console
    // Could show user notification here
  }
}

const formatNumber = (num: number): string => {
  return new Intl.NumberFormat().format(num)
}

// Initialize filters when dialog opens
watch(() => uiStore.showFilterDialog, (show) => {
  if (show) {
    // Load current filters from store
    const currentFilters = filterStore.currentFilters
    if (currentFilters) {
      activeFilters.value = {
        logLevels: currentFilters.logLevels || [],
        includeText: currentFilters.includePatterns?.map((p: any) => ({ pattern: p.pattern, type: p.type })) || [],
        excludeText: currentFilters.excludePatterns?.map((p: any) => ({ pattern: p.pattern, type: p.type })) || [],
        timeRange: currentFilters.timeRange || { start: '', end: '' }
      }
    }
    
    // Ensure at least one empty filter for each type
    if (activeFilters.value.includeText.length === 0) {
      activeFilters.value.includeText.push({ pattern: '', type: 'contains' })
    }
    if (activeFilters.value.excludeText.length === 0) {
      activeFilters.value.excludeText.push({ pattern: '', type: 'contains' })
    }
  }
})

onMounted(() => {
  // Initialize with empty filters
  addIncludeFilter()
  addExcludeFilter()
})
</script>

<style scoped>
.filter-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

:global(.theme-dark) .filter-dialog-overlay {
  background: rgba(0, 0, 0, 0.8);
}

.filter-dialog {
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  width: 90%;
  max-width: 900px;
  max-height: 85vh;
  overflow-y: auto;
  box-shadow: var(--shadow-lg);
}

/* Ensure theme variables are available in dialog */
.filter-dialog:not(.theme-dark) {
  --bg-primary: #ffffff;
  --bg-secondary: #f8f9fa;
  --bg-tertiary: #e9ecef;
  --bg-accent: #3498db;
  --bg-accent-subtle: rgba(52, 152, 219, 0.1);
  --bg-error: #e74c3c;
  --bg-error-hover: #c0392b;
  
  --text-primary: #2c3e50;
  --text-secondary: #6c757d;
  --text-muted: #adb5bd;
  --text-inverse: #ffffff;
  --text-error: #e74c3c;
  
  --border-primary: #dee2e6;
  --border-secondary: #ced4da;
  --border-focus: #80bdff;
  --focus-ring: rgba(0, 123, 255, 0.25);
  
  --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
}

.filter-dialog.theme-dark {
  --bg-primary: #1a1a1a;
  --bg-secondary: #2d3748;
  --bg-tertiary: #4a5568;
  --bg-accent: #3498db;
  --bg-accent-subtle: rgba(52, 152, 219, 0.1);
  --bg-error: #e53e3e;
  --bg-error-hover: #c53030;
  
  --text-primary: #f7fafc;
  --text-secondary: #e2e8f0;
  --text-muted: #a0aec0;
  --text-inverse: #1a1a1a;
  --text-error: #fc8181;
  
  --border-primary: #4a5568;
  --border-secondary: #2d3748;
  --border-focus: #63b3ed;
  --focus-ring: rgba(99, 179, 237, 0.25);
  
  --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.5);
}

.filter-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px;
  border-bottom: 1px solid var(--border-primary);
}

.filter-header h3 {
  margin: 0;
  font-size: 20px;
  color: var(--text-primary);
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.filter-content {
  padding: 24px;
}

.filter-section {
  margin-bottom: 32px;
}

.filter-section h4 {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0 0 16px 0;
  font-size: 16px;
  color: var(--text-primary);
  font-weight: 600;
}

.section-icon {
  font-size: 18px;
}

.log-level-filters {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 12px;
  margin-bottom: 16px;
}

.level-filter {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  border: 2px solid var(--border-primary);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--bg-secondary);
}

.level-filter.active {
  border-color: var(--border-focus);
  background: var(--bg-accent-subtle);
}

.level-filter:hover {
  background: var(--bg-tertiary);
}

.level-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
}

.level-name {
  font-weight: 500;
  color: var(--text-primary);
  flex: 1;
}

.level-count {
  font-size: 12px;
  color: var(--text-secondary);
  background: var(--bg-primary);
  padding: 2px 6px;
  border-radius: 10px;
}

.level-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.action-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background: var(--bg-accent);
  color: var(--text-inverse);
}

.text-filter-group {
  margin-bottom: 20px;
}

.filter-label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: var(--text-primary);
}

.filter-input-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.filter-input-item {
  display: flex;
  gap: 8px;
  align-items: center;
}

.filter-input {
  flex: 1;
  padding: 8px;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.filter-type-select {
  padding: 8px;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  min-width: 100px;
}

.remove-filter-btn {
  background: var(--bg-error);
  color: var(--text-inverse);
  border: none;
  border-radius: 4px;
  padding: 8px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.remove-filter-btn:hover {
  background: var(--bg-error-hover);
}

.add-filter-btn {
  padding: 8px 16px;
  border: 2px dashed var(--border-secondary);
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.add-filter-btn:hover {
  border-color: var(--border-focus);
  color: var(--text-primary);
}

.time-filter-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 16px;
}

.time-input-group label {
  display: block;
  margin-bottom: 4px;
  font-size: 14px;
  color: var(--text-primary);
}

.time-input {
  width: 100%;
  padding: 8px;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.time-presets {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.preset-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s ease;
}

.preset-btn:hover {
  background: var(--bg-accent);
  color: var(--text-inverse);
}

.filter-stats {
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 8px;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 16px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.stat-value {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.filter-footer {
  padding: 16px 24px;
  border-top: 1px solid var(--border-primary);
}

.filter-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.btn {
  padding: 8px 16px;
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.btn-secondary {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.btn-secondary:hover {
  background: var(--bg-tertiary);
}

.btn-primary {
  background: var(--bg-accent);
  color: var(--text-inverse);
  border-color: var(--bg-accent);
}

.btn-primary:hover:not(:disabled) {
  filter: brightness(1.1);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

/* Log level specific styling */
.level-error.active {
  border-color: #ef4444;
  background: rgba(239, 68, 68, 0.1);
}

.level-warn.active {
  border-color: #f59e0b;
  background: rgba(245, 158, 11, 0.1);
}

.level-info.active {
  border-color: #3b82f6;
  background: rgba(59, 130, 246, 0.1);
}

.level-debug.active {
  border-color: #6b7280;
  background: rgba(107, 114, 128, 0.1);
}

/* Responsive design */
@media (max-width: 768px) {
  .filter-dialog {
    width: 95%;
    max-height: 90vh;
  }
  
  .log-level-filters {
    grid-template-columns: 1fr;
  }
  
  .time-filter-grid {
    grid-template-columns: 1fr;
  }
  
  .filter-actions {
    justify-content: stretch;
    flex-direction: column;
  }
  
  .filter-stats {
    grid-template-columns: 1fr;
  }
}
</style>