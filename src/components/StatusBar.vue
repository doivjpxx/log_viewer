<template>
  <div class="status-bar">
    <!-- File info section -->
    <div class="status-section status-file">
      <template v-if="currentFile">
        <span class="status-item">
          <span class="status-icon">📄</span>
          <span>{{ currentFile.name }}</span>
        </span>
        
        <span class="status-item">
          <span class="status-icon">📏</span>
          <span>{{ formatFileSize(currentFile.size) }}</span>
        </span>
        
        <span class="status-item">
          <span class="status-icon">🔤</span>
          <span>{{ currentFile.encoding }}</span>
        </span>
        
        <span class="status-item">
          <span class="status-icon">📊</span>
          <span>{{ currentFile.lines.toLocaleString() }} lines</span>
        </span>
      </template>
      <span v-else class="status-item status-empty">
        No file open
      </span>
    </div>

    <!-- Position info section -->
    <div class="status-section status-position">
      <template v-if="currentFile">
        <span class="status-item">
          <span class="status-icon">🔍</span>
          <span>Line {{ currentLine + 1 }}</span>
        </span>
        
        <span class="status-item" v-if="selectedLinesCount > 0">
          <span class="status-icon">✅</span>
          <span>{{ selectedLinesCount }} selected</span>
        </span>
        
        <span class="status-item">
          <span class="status-icon">👁</span>
          <span>{{ visibleStartLine }}-{{ visibleEndLine }}</span>
        </span>
      </template>
    </div>

    <!-- Search info section -->
    <div class="status-section status-search">
      <template v-if="hasResults">
        <span class="status-item status-highlight">
          <span class="status-icon">🎯</span>
          <span>{{ totalResults }} matches</span>
        </span>
        
        <span class="status-item" v-if="currentResult">
          <span class="status-icon">📍</span>
          <span>Result {{ currentResultIndex + 1 }}/{{ totalResults }}</span>
        </span>
      </template>
      
      <span v-if="isSearching" class="status-item status-loading">
        <span class="status-icon spinner">🔄</span>
        <span>Searching...</span>
      </span>
    </div>

    <!-- Performance info section -->
    <div class="status-section status-performance">
      <template v-if="currentFile">
        <span class="status-item status-subtle">
          <span class="status-icon">⚡</span>
          <span>{{ loadTime }}ms</span>
        </span>
        
        <span class="status-item status-subtle" v-if="memoryUsage">
          <span class="status-icon">💾</span>
          <span>{{ memoryUsage }}</span>
        </span>
        
        <span class="status-item status-subtle">
          <span class="status-icon">🎮</span>
          <span>{{ fps }} FPS</span>
        </span>
      </template>
    </div>

    <!-- Activity indicator -->
    <div class="status-section status-activity">
      <div v-if="isLoading" class="activity-indicator">
        <div class="activity-spinner"></div>
        <span>Loading...</span>
      </div>
      
      <div v-else-if="error" class="activity-indicator activity-error">
        <span class="status-icon">⚠️</span>
        <span>Error</span>
      </div>
      
      <div v-else-if="currentFile" class="activity-indicator activity-ready">
        <span class="status-icon">✅</span>
        <span>Ready</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useFileStore } from '../stores/fileStore'
import { useSearchStore } from '../stores/searchStore'
import { useViewStore } from '../stores/viewStore'

// Stores
const fileStore = useFileStore()
const searchStore = useSearchStore()
const viewStore = useViewStore()

// Performance tracking
const loadTime = ref(0)
const memoryUsage = ref('')
const fps = ref(60)

// Computed
const currentFile = computed(() => fileStore.currentFile)
const isLoading = computed(() => fileStore.isLoading)
const error = computed(() => fileStore.error || searchStore.searchError)

const currentLine = computed(() => viewStore.navigation.currentLine)
const selectedLinesCount = computed(() => viewStore.navigation.selectedLines.size)

const visibleStartLine = computed(() => viewStore.viewport.startLine + 1)
const visibleEndLine = computed(() => viewStore.viewport.endLine + 1)

const hasResults = computed(() => searchStore.hasResults)
const totalResults = computed(() => searchStore.totalResults)
const currentResult = computed(() => searchStore.currentResult)
const currentResultIndex = computed(() => searchStore.currentResultIndex)
const isSearching = computed(() => searchStore.isSearching)

// Methods
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

onMounted(() => {
  // Simple performance tracking
  loadTime.value = Math.floor(Math.random() * 100) + 50 // Mock value
  memoryUsage.value = '25MB' // Mock value
})
</script>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 16px;
  background: #f8f9fa;
  border-top: 1px solid #e9ecef;
  font-size: 12px;
  line-height: 1.4;
  color: #495057;
  min-height: 32px;
  flex-wrap: wrap;
  gap: 8px;
}

.status-section {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 0 0 auto;
}

.status-file {
  flex: 1 1 auto;
  min-width: 0;
}

.status-position,
.status-search {
  flex: 0 1 auto;
}

.status-performance {
  flex: 0 0 auto;
}

.status-activity {
  flex: 0 0 auto;
}

.status-item {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px 6px;
  border-radius: 3px;
  white-space: nowrap;
  font-size: 12px;
}

.status-item:hover {
  background: rgba(0, 0, 0, 0.05);
}

.status-empty {
  color: #6c757d;
  font-style: italic;
}

.status-highlight {
  background: #fff3cd;
  color: #856404;
}

.status-loading {
  background: #d1ecf1;
  color: #0c5460;
}

.status-subtle {
  color: #6c757d;
  font-size: 11px;
}

.status-icon {
  font-size: 10px;
  opacity: 0.8;
}

.activity-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 500;
}

.activity-ready {
  background: #d4edda;
  color: #155724;
}

.activity-error {
  background: #f8d7da;
  color: #721c24;
}

.activity-indicator:has(.activity-spinner) {
  background: #d1ecf1;
  color: #0c5460;
}

.activity-spinner {
  width: 12px;
  height: 12px;
  border: 2px solid #b6e0e6;
  border-top: 2px solid #0c5460;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* Responsive design */
@media (max-width: 768px) {
  .status-bar {
    font-size: 11px;
    padding: 4px 12px;
    gap: 6px;
  }

  .status-section {
    gap: 8px;
  }

  .status-performance,
  .status-subtle {
    display: none;
  }
}

@media (max-width: 480px) {
  .status-bar {
    flex-direction: column;
    align-items: stretch;
    gap: 4px;
  }

  .status-section {
    justify-content: center;
  }
}
</style>