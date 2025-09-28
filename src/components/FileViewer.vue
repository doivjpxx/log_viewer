<template>
  <div class="file-viewer">
    <!-- File Operations Toolbar -->
    <div class="toolbar bg-gray-100 p-4 border-b">
      <div class="flex items-center gap-4">
        <button 
          @click="openFileDialog" 
          :disabled="isLoading"
          class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
        >
          {{ isLoading ? 'Loading...' : 'Open File' }}
        </button>
        
        <div v-if="currentFile" class="file-info flex items-center gap-4 text-sm">
          <span class="font-medium">{{ currentFile.name }}</span>
          <span class="text-gray-600">{{ formatFileSize(currentFile.size) }}</span>
          <span class="text-gray-600">{{ currentFile.encoding }}</span>
          <span class="text-gray-600">{{ currentFile.lines.toLocaleString() }} lines</span>
        </div>
      </div>

      <!-- Search Bar -->
      <div v-if="currentFile" class="mt-4">
        <div class="flex items-center gap-2">
          <input
            v-model="searchQuery"
            @keyup.enter="performSearch"
            placeholder="Search in file..."
            class="flex-1 px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            @click="performSearch"
            :disabled="isSearching || !searchQuery.trim()"
            class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50"
          >
            {{ isSearching ? 'Searching...' : 'Search' }}
          </button>
          <button
            @click="clearSearch"
            :disabled="!hasResults"
            class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 disabled:opacity-50"
          >
            Clear
          </button>
        </div>
        
        <!-- Search Options -->
        <div class="flex items-center gap-4 mt-2">
          <label class="flex items-center gap-1">
            <input 
              v-model="searchOptions.is_case_sensitive" 
              type="checkbox" 
              class="rounded"
            />
            <span class="text-sm">Case sensitive</span>
          </label>
          <label class="flex items-center gap-1">
            <input 
              v-model="searchOptions.is_regex" 
              type="checkbox" 
              class="rounded"
            />
            <span class="text-sm">Regex</span>
          </label>
          <label class="flex items-center gap-1">
            <input 
              v-model="searchOptions.is_whole_word" 
              type="checkbox" 
              class="rounded"
            />
            <span class="text-sm">Whole word</span>
          </label>
        </div>

        <!-- Search Results Info -->
        <div v-if="hasResults" class="flex items-center gap-4 mt-2 text-sm text-gray-600">
          <span>{{ totalResults }} results found</span>
          <div v-if="currentResult" class="flex items-center gap-2">
            <button
              @click="previousResult"
              :disabled="currentResultIndex <= 0"
              class="px-2 py-1 bg-gray-200 rounded hover:bg-gray-300 disabled:opacity-50"
            >
              ↑
            </button>
            <span>{{ currentResultIndex + 1 }} / {{ totalResults }}</span>
            <button
              @click="nextResult"
              :disabled="currentResultIndex >= totalResults - 1"
              class="px-2 py-1 bg-gray-200 rounded hover:bg-gray-300 disabled:opacity-50"
            >
              ↓
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Error Display -->
    <div v-if="error || searchError" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3">
      <p class="font-semibold">Error:</p>
      <p>{{ error || searchError }}</p>
      <button @click="clearErrors" class="mt-2 text-sm underline">Clear</button>
    </div>

    <!-- File Content Display -->
    <div v-if="currentFile" class="file-content flex-1 overflow-hidden">
      <div class="h-64 p-4 bg-gray-50 border overflow-auto font-mono text-sm">
        <!-- Demo: Show first chunk of file -->
        <div v-if="currentChunk">
          <div 
            v-for="(line, index) in currentChunk.content" 
            :key="currentChunk.startLine + index"
            class="line flex"
            :class="{ 'bg-yellow-200': isLineHighlighted(currentChunk.startLine + index) }"
          >
            <span class="line-number w-16 text-gray-500 text-right pr-2 select-none">
              {{ currentChunk.startLine + index + 1 }}
            </span>
            <span class="line-content" v-html="highlightSearchResults(line)"></span>
          </div>
        </div>
        <div v-else class="text-gray-500 text-center">
          <p>No content loaded. Click "Load First 50 Lines" to start.</p>
          <button 
            @click="loadInitialChunk" 
            :disabled="isLoading"
            class="mt-2 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
          >
            Load First 50 Lines
          </button>
        </div>
      </div>
    </div>

    <!-- Welcome Message -->
    <div v-else class="flex-1 flex items-center justify-center text-gray-500">
      <div class="text-center">
        <h2 class="text-xl mb-2">Log File Viewer</h2>
        <p>Click "Open File" to get started</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useFileStore } from '../stores/fileStore'
import { useSearchStore } from '../stores/searchStore'

// Stores
const fileStore = useFileStore()
const searchStore = useSearchStore()

// Local state
const searchQuery = ref('')
const currentChunk = ref<any>(null)

// Computed from stores
const currentFile = computed(() => fileStore.currentFile)
const isLoading = computed(() => fileStore.isLoading)
const error = computed(() => fileStore.error)
const searchError = computed(() => searchStore.searchError)
const isSearching = computed(() => searchStore.isSearching)
const hasResults = computed(() => searchStore.hasResults)
const totalResults = computed(() => searchStore.totalResults)
const currentResult = computed(() => searchStore.currentResult)
const currentResultIndex = computed(() => searchStore.currentResultIndex)
const searchOptions = computed(() => searchStore.searchOptions)

// Methods
async function openFileDialog() {
  try {
    const filePath = await fileStore.openFileDialog()
    if (filePath) {
      await fileStore.openFile(filePath)
      // Load initial chunk after opening file
      loadInitialChunk()
    }
  } catch {
    // Error will be handled by the store and displayed in the UI
  }
}

async function loadInitialChunk() {
  if (!currentFile.value) return
  
  try {
    await fileStore.loadFileChunk(currentFile.value.id, 0, 49) // Load first 50 lines
    const chunks = fileStore.getFileChunks(currentFile.value.id)
    if (chunks.length > 0) {
      currentChunk.value = chunks[0]
    }
  } catch {
    // Error will be handled by the store and displayed in the UI
  }
}

async function performSearch() {
  if (!searchQuery.value.trim()) return
  
  try {
    await searchStore.search(searchQuery.value, searchOptions.value)
  } catch {
    // Error will be handled by the store and displayed in the UI
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

function clearErrors() {
  fileStore.clearError()
  if (searchStore.searchError) {
    // Clear search error by performing an empty search
    searchStore.clearSearch()
  }
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function isLineHighlighted(lineNumber: number): boolean {
  return currentResult.value?.line_number === lineNumber
}

function highlightSearchResults(content: string): string {
  if (!hasResults.value || !searchQuery.value) {
    return content
  }
  
  // Simple highlighting - escape HTML and highlight search query
  const escapedContent = content
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
  
  const query = searchQuery.value
  if (!searchOptions.value.is_regex) {
    const flags = searchOptions.value.is_case_sensitive ? 'g' : 'gi'
    const regex = new RegExp(query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), flags)
    return escapedContent.replace(regex, '<mark class="bg-yellow-300">$&</mark>')
  }
  
  return escapedContent
}

// Watch for file changes and clear chunk
watch(currentFile, (newFile) => {
  if (!newFile) {
    currentChunk.value = null
  }
})
</script>

<style scoped>
.file-viewer {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.line:hover {
  background-color: #f8f9fa;
}

.line-number {
  user-select: none;
}
</style>