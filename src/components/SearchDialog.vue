<template>
  <div v-if="showDialog" class="search-dialog-overlay" @click="closeDialog">
    <div class="search-dialog" :class="{ 'theme-dark': uiStore.isDarkTheme }" @click.stop>
      <div class="search-header">
        <h3>Advanced Search</h3>
        <button class="close-btn" @click="closeDialog">×</button>
      </div>

      <div class="search-content">
        <!-- Main search input -->
        <div class="search-input-group">
          <label for="search-query">Search Query</label>
          <div class="input-with-suggestions">
            <input
              id="search-query"
              ref="searchInput"
              v-model="searchQuery"
              type="text"
              placeholder="Enter search term or regex pattern..."
              class="search-input"
              @keydown.enter="handleSearch"
              @keydown.up="navigateSuggestions(-1)"
              @keydown.down="navigateSuggestions(1)"
              @keydown.tab="selectSuggestion"
              @input="showSuggestions = true"
              @focus="showSuggestions = searchQuery.length > 0"
              @blur="hideSuggestionsDelayed"
            />
            
            <!-- Search suggestions -->
            <div v-if="showSuggestions && filteredSuggestions.length > 0" class="suggestions-dropdown">
              <div
                v-for="(suggestion, index) in filteredSuggestions"
                :key="index"
                class="suggestion-item"
                :class="{ active: index === selectedSuggestionIndex }"
                @click="applySuggestion(suggestion)"
              >
                {{ suggestion }}
              </div>
            </div>
          </div>
        </div>

        <!-- Search options -->
        <div class="search-options-grid">
          <div class="option-group">
            <h4>Search Type</h4>
            <label class="checkbox-option">
              <input
                v-model="searchOptions.is_regex"
                type="checkbox"
                @change="validateRegex"
              />
              <span class="checkbox-label">Regular Expression</span>
            </label>
            <label class="checkbox-option">
              <input v-model="searchOptions.is_case_sensitive" type="checkbox" />
              <span class="checkbox-label">Case Sensitive</span>
            </label>
            <label class="checkbox-option">
              <input v-model="searchOptions.is_whole_word" type="checkbox" />
              <span class="checkbox-label">Whole Words Only</span>
            </label>
          </div>

          <div class="option-group">
            <h4>Search Scope</h4>
            <div class="number-input">
              <label for="max-results">Max Results</label>
              <input
                id="max-results"
                v-model.number="searchOptions.max_results"
                type="number"
                min="1"
                max="10000"
                class="number-field"
              />
            </div>
            <div class="number-input">
              <label for="context-lines">Context Lines</label>
              <input
                id="context-lines"
                v-model.number="contextLines"
                type="number"
                min="0"
                max="10"
                class="number-field"
              />
            </div>
          </div>

          <div class="option-group">
            <h4>Search Range</h4>
            <div class="range-inputs">
              <div class="number-input">
                <label for="start-line">Start Line</label>
                <input
                  id="start-line"
                  v-model.number="startLine"
                  type="number"
                  min="1"
                  :max="fileStore.currentFileLines"
                  class="number-field"
                />
              </div>
              <div class="number-input">
                <label for="end-line">End Line</label>
                <input
                  id="end-line"
                  v-model.number="endLine"
                  type="number"
                  :min="startLine || 1"
                  :max="fileStore.currentFileLines"
                  class="number-field"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Regex validation error -->
        <div v-if="regexError" class="error-message">
          <span class="error-icon">⚠️</span>
          {{ regexError }}
        </div>

        <!-- Search progress -->
        <div v-if="searchStore.isSearching" class="search-progress">
          <div class="progress-info">
            <span>Searching...</span>
            <span class="matches-count">{{ searchStore.totalResults }} matches found</span>
          </div>
          <div class="progress-bar">
            <div 
              class="progress-fill"
              :style="{ width: searchProgress + '%' }"
            />
          </div>
        </div>

        <!-- Search results summary -->
        <div v-if="searchStore.hasResults && !searchStore.isSearching" class="results-summary">
          <div class="results-info">
            <span class="results-count">{{ searchStore.totalResults }} matches found</span>
            <span class="search-time">in {{ searchTime }}ms</span>
          </div>
          
          <!-- Navigation controls -->
          <div class="navigation-controls">
            <button
              class="nav-btn"
              :disabled="!canNavigatePrev"
              @click="navigateToResult('prev')"
              title="Previous match (Shift+F3)"
            >
              ↑ Previous
            </button>
            <span class="current-match">
              {{ currentMatchIndex + 1 }} / {{ searchStore.totalResults }}
            </span>
            <button
              class="nav-btn"
              :disabled="!canNavigateNext"
              @click="navigateToResult('next')"
              title="Next match (F3)"
            >
              ↓ Next
            </button>
          </div>
        </div>
      </div>

      <div class="search-footer">
        <div class="footer-buttons">
          <button class="btn btn-secondary" @click="clearSearch">
            Clear
          </button>
          <button
            class="btn btn-primary"
            :disabled="!canSearch"
            @click="handleSearch"
          >
            {{ searchStore.isSearching ? 'Searching...' : 'Search' }}
          </button>
        </div>

        <!-- Keyboard shortcuts hint -->
        <div class="shortcuts-hint">
          <span>Press <kbd>Enter</kbd> to search, <kbd>Esc</kbd> to close</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue'
import { useSearchStore } from '../stores/searchStore'
import { useFileStore } from '../stores/fileStore'
import { useUIStore } from '../stores/uiStore'

const searchStore = useSearchStore()
const fileStore = useFileStore()
const uiStore = useUIStore()

// Refs
const searchInput = ref<HTMLInputElement | null>(null)

// State
const searchQuery = ref('')
const showSuggestions = ref(false)
const selectedSuggestionIndex = ref(-1)
const contextLines = ref(2)
const startLine = ref<number | null>(null)
const endLine = ref<number | null>(null)
const regexError = ref<string | null>(null)
const searchTime = ref(0)
const searchStartTime = ref(0)

const searchOptions = ref({
  is_regex: false,
  is_case_sensitive: false,
  is_whole_word: false,
  max_results: 1000,
})

// Computed
const showDialog = computed(() => uiStore.showSearchDialog)

const filteredSuggestions = computed(() => {
  if (!searchQuery.value.trim()) return []
  
  return searchStore.searchHistory
    .filter(query => 
      query.toLowerCase().includes(searchQuery.value.toLowerCase()) &&
      query !== searchQuery.value
    )
    .slice(0, 8)
})

const canSearch = computed(() => {
  return searchQuery.value.trim().length > 0 && 
         fileStore.hasOpenFile &&
         !regexError.value &&
         !searchStore.isSearching
})

const searchProgress = computed(() => {
  // Simulated progress - in real implementation this would come from backend
  return searchStore.isSearching ? Math.min(90, searchStartTime.value ? 
    ((Date.now() - searchStartTime.value) / 50) : 0) : 0
})

const currentMatchIndex = computed(() => searchStore.currentResultIndex)
const canNavigatePrev = computed(() => currentMatchIndex.value > 0)
const canNavigateNext = computed(() => currentMatchIndex.value < searchStore.totalResults - 1)

// Methods
const closeDialog = () => {
  uiStore.toggleSearchDialog()
}

const validateRegex = () => {
  regexError.value = null
  
  if (searchOptions.value.is_regex && searchQuery.value) {
    try {
      new RegExp(searchQuery.value)
    } catch (error) {
      regexError.value = `Invalid regex: ${(error as Error).message}`
    }
  }
}

const handleSearch = async () => {
  if (!canSearch.value) return

  try {
    searchStartTime.value = Date.now()
    searchTime.value = 0
    regexError.value = null

    const options = {
      ...searchOptions.value,
      query: searchQuery.value,
      // Add range support if implemented in backend
      // start_line: startLine.value,
      // end_line: endLine.value,
    }

    await searchStore.search(searchQuery.value, options)
    
    searchTime.value = Date.now() - searchStartTime.value
    
    // Close dialog after successful search
    if (searchStore.hasResults) {
      closeDialog()
    }
  } catch (error) {
    // eslint-disable-next-line no-console
    console.error('Search failed:', error)
    regexError.value = `Search failed: ${(error as Error).message}`
  }
}

const clearSearch = () => {
  searchStore.clearSearch()
  searchQuery.value = ''
  showSuggestions.value = false
  regexError.value = null
}

const navigateToResult = (direction: 'prev' | 'next') => {
  if (direction === 'next') {
    searchStore.nextResult()
  } else {
    searchStore.previousResult()
  }
}

const navigateSuggestions = (direction: number) => {
  if (!showSuggestions.value || filteredSuggestions.value.length === 0) return
  
  selectedSuggestionIndex.value = Math.max(0, 
    Math.min(filteredSuggestions.value.length - 1, 
      selectedSuggestionIndex.value + direction
    )
  )
}

const selectSuggestion = () => {
  if (selectedSuggestionIndex.value >= 0 && 
      filteredSuggestions.value[selectedSuggestionIndex.value]) {
    applySuggestion(filteredSuggestions.value[selectedSuggestionIndex.value])
  }
}

const applySuggestion = (suggestion: string) => {
  searchQuery.value = suggestion
  showSuggestions.value = false
  selectedSuggestionIndex.value = -1
  nextTick(() => searchInput.value?.focus())
}

const hideSuggestionsDelayed = () => {
  if (typeof globalThis !== 'undefined' && globalThis.setTimeout) {
    globalThis.setTimeout(() => {
      showSuggestions.value = false
      selectedSuggestionIndex.value = -1
    }, 200)
  }
}

// Global keyboard handlers
const handleGlobalKeydown = (event: unknown) => {
  if (!showDialog.value) return
  
  const keyboardEvent = event as { key: string; shiftKey: boolean; preventDefault: () => void }

  if (keyboardEvent.key === 'Escape') {
    keyboardEvent.preventDefault()
    closeDialog()
  } else if (keyboardEvent.key === 'F3') {
    keyboardEvent.preventDefault()
    if (keyboardEvent.shiftKey) {
      navigateToResult('prev')
    } else {
      navigateToResult('next')
    }
  }
}

// Watchers
watch(searchQuery, () => {
  if (searchOptions.value.is_regex) {
    validateRegex()
  }
})

watch(() => uiStore.showSearchDialog, (show) => {
  if (show) {
    nextTick(() => {
      searchInput.value?.focus()
      // Restore previous search if exists
      if (searchStore.searchOptions.query) {
        searchQuery.value = searchStore.searchOptions.query
        Object.assign(searchOptions.value, {
          is_regex: searchStore.searchOptions.is_regex,
          is_case_sensitive: searchStore.searchOptions.is_case_sensitive,
          is_whole_word: searchStore.searchOptions.is_whole_word,
          max_results: searchStore.searchOptions.max_results,
        })
      }
    })
  }
})

onMounted(() => {
  if (typeof globalThis !== 'undefined' && globalThis.document) {
    globalThis.document.addEventListener('keydown', handleGlobalKeydown)
  }
})

onUnmounted(() => {
  if (typeof globalThis !== 'undefined' && globalThis.document) {
    globalThis.document.removeEventListener('keydown', handleGlobalKeydown)
  }
})
</script>

<style scoped>
.search-dialog-overlay {
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

:global(.theme-dark) .search-dialog-overlay {
  background: rgba(0, 0, 0, 0.8);
}

.search-dialog {
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  width: 90%;
  max-width: 800px;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: var(--shadow-lg);
}

/* Ensure theme variables are available in dialog */
.search-dialog:not(.theme-dark) {
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

.search-dialog.theme-dark {
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

.search-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px 24px 0 24px;
  border-bottom: 1px solid var(--border-primary);
  margin-bottom: 24px;
}

.search-header h3 {
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

.search-content {
  padding: 0 24px 24px 24px;
}

.search-input-group {
  margin-bottom: 24px;
}

.search-input-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: var(--text-primary);
}

.input-with-suggestions {
  position: relative;
}

.search-input {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  font-family: 'JetBrains Mono', monospace;
}

.search-input:focus {
  outline: none;
  border-color: var(--border-focus);
  box-shadow: 0 0 0 3px var(--focus-ring);
}

.suggestions-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  box-shadow: var(--shadow-md);
  z-index: 10;
  max-height: 200px;
  overflow-y: auto;
}

.suggestion-item {
  padding: 8px 12px;
  cursor: pointer;
  transition: background-color 0.2s ease;
  font-family: 'JetBrains Mono', monospace;
  font-size: 14px;
}

.suggestion-item:hover,
.suggestion-item.active {
  background: var(--bg-accent);
  color: var(--text-inverse);
}

.search-options-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 24px;
  margin-bottom: 24px;
}

.option-group h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  color: var(--text-primary);
  font-weight: 600;
}

.checkbox-option {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
  cursor: pointer;
}

.checkbox-option input[type="checkbox"] {
  margin-right: 8px;
  cursor: pointer;
}

.checkbox-label {
  font-size: 14px;
  color: var(--text-primary);
}

.number-input {
  margin-bottom: 12px;
}

.number-input label {
  display: block;
  margin-bottom: 4px;
  font-size: 14px;
  color: var(--text-primary);
}

.number-field {
  width: 100%;
  padding: 8px;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.range-inputs {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.error-message {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: var(--bg-error);
  color: var(--text-error);
  border-radius: 6px;
  margin-bottom: 16px;
  font-size: 14px;
}

.search-progress {
  margin-bottom: 16px;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
  font-size: 14px;
  color: var(--text-secondary);
}

.matches-count {
  font-weight: 500;
  color: var(--text-primary);
}

.progress-bar {
  height: 6px;
  background: var(--bg-secondary);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--bg-accent);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.results-summary {
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 8px;
  margin-bottom: 16px;
}

.results-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.results-count {
  font-weight: 600;
  color: var(--text-primary);
}

.search-time {
  font-size: 12px;
  color: var(--text-secondary);
}

.navigation-controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
}

.nav-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s ease;
}

.nav-btn:hover:not(:disabled) {
  background: var(--bg-accent);
  color: var(--text-inverse);
}

.nav-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.current-match {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 500;
  min-width: 80px;
  text-align: center;
}

.search-footer {
  padding: 16px 24px;
  border-top: 1px solid var(--border-primary);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.footer-buttons {
  display: flex;
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

.shortcuts-hint {
  font-size: 12px;
  color: var(--text-secondary);
}

kbd {
  background: var(--bg-secondary);
  border: 1px solid var(--border-secondary);
  border-radius: 3px;
  padding: 2px 6px;
  font-size: 11px;
  font-family: monospace;
}

/* Responsive design */
@media (max-width: 768px) {
  .search-dialog {
    width: 95%;
    max-height: 90vh;
  }
  
  .search-options-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }
  
  .range-inputs {
    grid-template-columns: 1fr;
  }
  
  .navigation-controls {
    flex-direction: column;
    gap: 8px;
  }
  
  .results-info {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }
}
</style>