<template>
  <div class="file-toolbar">
    <!-- Primary actions -->
    <div class="toolbar-section toolbar-primary">
      <button 
        @click="openFile" 
        :disabled="isLoading"
        class="btn btn-primary"
        title="Open file (Ctrl+O)"
      >
        <span class="btn-icon">📁</span>
        <span class="btn-text">Open File</span>
      </button>
      
      <button 
        @click="reloadFile" 
        :disabled="!currentFile || isLoading"
        class="btn btn-secondary"
        title="Reload file (F5)"
      >
        <span class="btn-icon">🔄</span>
        <span class="btn-text">Reload</span>
      </button>
      
      <div class="separator"></div>
      
      <!-- File tabs -->
      <div class="file-tabs" v-if="openFiles.length > 0">
        <div 
          v-for="file in openFiles"
          :key="file.id"
          :class="['file-tab', { 'file-tab--active': file.id === currentFile?.id }]"
          @click="switchToFile(file.id)"
        >
          <span class="file-tab__name">{{ file.name }}</span>
          <button 
            class="file-tab__close"
            @click.stop="closeFile(file.id)"
            title="Close file"
          >
            ×
          </button>
        </div>
      </div>
    </div>

    <!-- Search section -->
    <div class="toolbar-section toolbar-search">
      <div class="search-input-group">
        <input
          v-model="searchQuery"
          @keyup.enter="performSearch"
          @keyup.escape="clearSearch"
          placeholder="Search in file... (Ctrl+F)"
          class="search-input"
          :disabled="!currentFile"
        />
        <button
          @click="performSearch"
          :disabled="isSearching || !searchQuery.trim() || !currentFile"
          class="btn btn-search"
          title="Search"
        >
          🔍
        </button>
        <button
          @click="uiStore.toggleSearchDialog"
          :disabled="!currentFile"
          class="btn btn-advanced"
          title="Advanced Search (Ctrl+Shift+F)"
        >
          🔍+
        </button>
        <button
          @click="uiStore.toggleFilterDialog"
          :disabled="!currentFile"
          class="btn btn-filter"
          title="Filter Lines"
        >
          🔎
        </button>
        <button
          @click="uiStore.toggleParserConfig"
          :disabled="!currentFile"
          class="btn btn-config"
          title="Parser Configuration"
        >
          ⚙️
        </button>
        <button
          @click="clearSearch"
          :disabled="!hasResults"
          class="btn btn-clear"
          title="Clear search"
        >
          ✕
        </button>
      </div>

      <!-- Search options -->
      <div class="search-options" v-if="currentFile">
        <label class="search-option">
          <input 
            v-model="searchOptions.is_case_sensitive" 
            type="checkbox" 
          />
          <span title="Case sensitive">Aa</span>
        </label>
        <label class="search-option">
          <input 
            v-model="searchOptions.is_regex" 
            type="checkbox" 
          />
          <span title="Regular expression">.*</span>
        </label>
        <label class="search-option">
          <input 
            v-model="searchOptions.is_whole_word" 
            type="checkbox" 
          />
          <span title="Whole word">Ab</span>
        </label>
      </div>

      <!-- Search results -->
      <div class="search-results" v-if="hasResults">
        <span class="search-count">{{ totalResults }} results</span>
        <div class="search-navigation">
          <button
            @click="previousResult"
            :disabled="currentResultIndex <= 0"
            class="btn btn-nav"
            title="Previous result (F3)"
          >
            ↑
          </button>
          <span class="search-position">{{ currentResultIndex + 1 }}/{{ totalResults }}</span>
          <button
            @click="nextResult"
            :disabled="currentResultIndex >= totalResults - 1"
            class="btn btn-nav"
            title="Next result (Shift+F3)"
          >
            ↓
          </button>
        </div>
      </div>
    </div>

    <!-- Navigation section -->
    <div class="toolbar-section toolbar-navigation">
      <button
        @click="showJumpToLineDialog"
        :disabled="!currentFile"
        class="btn btn-nav"
        title="Go to line (Ctrl+G)"
      >
        <span class="btn-icon">⤴</span>
        <span class="btn-text">Go to Line</span>
      </button>
      
      <div class="separator"></div>
      
      <button
        @click="scrollToTop"
        :disabled="!currentFile"
        class="btn btn-nav"
        title="Go to top (Ctrl+Home)"
      >
        ⬆
      </button>
      
      <button
        @click="scrollToBottom"
        :disabled="!currentFile"
        class="btn btn-nav"
        title="Go to bottom (Ctrl+End)"
      >
        ⬇
      </button>
      
      <div class="separator"></div>
      
      <!-- Theme toggle -->
      <button
        @click="toggleTheme"
        class="btn btn-nav"
        :title="uiStore.isDarkTheme ? 'Switch to Light Theme' : 'Switch to Dark Theme'"
      >
        {{ uiStore.isDarkTheme ? '☀️' : '🌙' }}
      </button>
    </div>
  </div>

  <!-- Jump to Line Dialog -->
  <div v-if="showJumpDialog" class="modal-overlay" @click="closeJumpDialog">
    <div class="modal-content" @click.stop>
      <h3>Go to Line</h3>
      <div class="form-group">
        <label>Line number:</label>
        <input
          v-model.number="jumpToLineNumber"
          ref="jumpInputRef"
          type="number"
          :min="1"
          :max="currentFile?.lines || 1"
          class="form-input"
          @keyup.enter="performJumpToLine"
          @keyup.escape="closeJumpDialog"
        />
        <small class="form-hint">
          Enter a line number between 1 and {{ currentFile?.lines?.toLocaleString() || 0 }}
        </small>
      </div>
      <div class="form-actions">
        <button @click="performJumpToLine" class="btn btn-primary">Go</button>
        <button @click="closeJumpDialog" class="btn btn-secondary">Cancel</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { useFileStore } from '../stores/fileStore'
import { useSearchStore } from '../stores/searchStore'
import { useViewStore } from '../stores/viewStore'
import { useUIStore } from '../stores/uiStore'

// Stores
const fileStore = useFileStore()
const searchStore = useSearchStore()
const viewStore = useViewStore()
const uiStore = useUIStore()

// Local state
const searchQuery = ref('')
const showJumpDialog = ref(false)
const jumpToLineNumber = ref(1)
const jumpInputRef = ref<any>()

// Computed from stores
const currentFile = computed(() => fileStore.currentFile)
const openFiles = computed(() => fileStore.openFiles)
const isLoading = computed(() => fileStore.isLoading)
const isSearching = computed(() => searchStore.isSearching)
const hasResults = computed(() => searchStore.hasResults)
const totalResults = computed(() => searchStore.totalResults)
const currentResultIndex = computed(() => searchStore.currentResultIndex)
const searchOptions = computed(() => searchStore.searchOptions)

// Methods
async function openFile() {
  try {
    const filePath = await fileStore.openFileDialog()
    if (filePath) {
      await fileStore.openFile(filePath)
    }
  } catch {
    // Error handled by store
  }
}

async function reloadFile() {
  if (currentFile.value) {
    await fileStore.openFile(currentFile.value.path)
  }
}

function switchToFile(fileId: string) {
  fileStore.switchToFile(fileId)
}

function closeFile(fileId: string) {
  fileStore.closeFile(fileId)
}

async function performSearch() {
  if (!searchQuery.value.trim()) return
  
  try {
    await searchStore.search(searchQuery.value, searchOptions.value)
  } catch {
    // Error handled by store
  }
}

function clearSearch() {
  searchStore.clearSearch()
  searchQuery.value = ''
}

function nextResult() {
  searchStore.nextResult()
}

function previousResult() {
  searchStore.previousResult()
}

function showJumpToLineDialog() {
  if (!currentFile.value) return
  
  showJumpDialog.value = true
  jumpToLineNumber.value = Math.max(1, viewStore.navigation.currentLine + 1)
  
  nextTick(() => {
    if (jumpInputRef.value) {
      jumpInputRef.value.focus()
      jumpInputRef.value.select()
    }
  })
}

function closeJumpDialog() {
  showJumpDialog.value = false
}

function performJumpToLine() {
  if (!currentFile.value) return
  
  const lineNumber = Math.max(1, Math.min(jumpToLineNumber.value, currentFile.value.lines))
  viewStore.jumpToLine(lineNumber - 1) // Convert to 0-based index
  closeJumpDialog()
}

function scrollToTop() {
  viewStore.scrollToTop()
}

function scrollToBottom() {
  viewStore.scrollToBottom()
}

function toggleTheme() {
  const currentTheme = uiStore.settings.theme
  let newTheme: 'auto' | 'light' | 'dark'
  
  if (currentTheme === 'auto') {
    newTheme = 'dark'
  } else if (currentTheme === 'dark') {
    newTheme = 'light'
  } else {
    newTheme = 'auto'
  }
  
  uiStore.updateSettings({ theme: newTheme })
}
</script>

<style scoped>
.file-toolbar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 16px;
  background: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
  flex-wrap: wrap;
}

.toolbar-section {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toolbar-primary {
  flex: 0 0 auto;
}

.toolbar-search {
  flex: 1 1 auto;
  justify-content: center;
  max-width: 600px;
}

.toolbar-navigation {
  flex: 0 0 auto;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  border: 1px solid #dee2e6;
  border-radius: 4px;
  background: #ffffff;
  color: #495057;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn:hover:not(:disabled) {
  background: #e9ecef;
  border-color: #adb5bd;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background: #007bff;
  border-color: #007bff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #0056b3;
  border-color: #004085;
}

.btn-secondary {
  background: #6c757d;
  border-color: #6c757d;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background: #545b62;
  border-color: #4e555b;
}

.btn-search {
  border-radius: 0 4px 4px 0;
  border-left: none;
}

.btn-clear {
  border-radius: 0;
  border-left: none;
}

.btn-config {
  background: #17a2b8;
  border-color: #17a2b8;
  color: white;
  border-radius: 0;
  border-left: none;
}

.btn-config:hover:not(:disabled) {
  background: #138496;
  border-color: #117a8b;
}

.btn-nav {
  min-width: 32px;
  justify-content: center;
}

.btn-icon {
  font-size: 14px;
}

.btn-text {
  font-size: 13px;
}

.separator {
  width: 1px;
  height: 24px;
  background: #dee2e6;
  margin: 0 8px;
}

/* File tabs */
.file-tabs {
  display: flex;
  gap: 2px;
  max-width: 300px;
  overflow-x: auto;
}

.file-tab {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 8px;
  background: #e9ecef;
  border: 1px solid #dee2e6;
  border-radius: 4px 4px 0 0;
  font-size: 12px;
  cursor: pointer;
  white-space: nowrap;
  min-width: 0;
}

.file-tab--active {
  background: #ffffff;
  border-bottom-color: #ffffff;
}

.file-tab__name {
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-tab__close {
  background: none;
  border: none;
  font-size: 16px;
  line-height: 1;
  cursor: pointer;
  padding: 0;
  color: #6c757d;
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 2px;
}

.file-tab__close:hover {
  background: #dee2e6;
  color: #495057;
}

/* Search */
.search-input-group {
  display: flex;
  align-items: center;
}

.search-input {
  padding: 6px 10px;
  border: 1px solid #ced4da;
  border-radius: 4px 0 0 4px;
  font-size: 13px;
  width: 240px;
  outline: none;
}

.search-input:focus {
  border-color: #80bdff;
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
}

.search-input:disabled {
  background: #e9ecef;
  opacity: 0.6;
}

.search-options {
  display: flex;
  gap: 4px;
  margin-left: 8px;
}

.search-option {
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 4px 6px;
  border: 1px solid #dee2e6;
  border-radius: 3px;
  background: #ffffff;
  cursor: pointer;
  font-size: 11px;
  font-weight: 500;
}

.search-option:has(input:checked) {
  background: #e3f2fd;
  border-color: #2196f3;
  color: #1976d2;
}

.search-option input {
  display: none;
}

.search-results {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: 8px;
}

.search-count {
  font-size: 12px;
  color: #6c757d;
}

.search-navigation {
  display: flex;
  align-items: center;
  gap: 4px;
}

.search-position {
  font-size: 12px;
  color: #6c757d;
  min-width: 40px;
  text-align: center;
}

/* Modal */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  padding: 24px;
  min-width: 320px;
  max-width: 90vw;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
}

.modal-content h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
  color: #212529;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 4px;
  font-size: 14px;
  font-weight: 500;
  color: #495057;
}

.form-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 14px;
  outline: none;
}

.form-input:focus {
  border-color: #80bdff;
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
}

.form-hint {
  display: block;
  margin-top: 4px;
  font-size: 12px;
  color: #6c757d;
}

.form-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

/* Responsive design */
@media (max-width: 768px) {
  .file-toolbar {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }

  .toolbar-section {
    justify-content: center;
  }

  .toolbar-search {
    max-width: none;
  }

  .search-input {
    width: 200px;
  }
}
</style>