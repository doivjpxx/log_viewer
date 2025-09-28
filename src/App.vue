<script setup lang="ts">
import { ref, reactive } from 'vue';
import { useFileStore, useSearchStore, useUIStore } from './stores';

const fileStore = useFileStore();
const searchStore = useSearchStore();
const uiStore = useUIStore();

const searchQuery = ref('');
const searchOptions = reactive({
  isRegex: false,
  isCaseSensitive: false,
});

// Mock file opening for now
const openFile = async () => {
  try {
    // This will be replaced with actual Tauri dialog
    const mockPath = `/tmp/sample-${Date.now()}.log`;
    await fileStore.openFile(mockPath);
  } catch (error) {
    console.error('Failed to open file:', error);
  }
};

const performSearch = async () => {
  if (!searchQuery.value.trim()) {
    return;
  }

  await searchStore.search(searchQuery.value, {
    isRegex: searchOptions.isRegex,
    isCaseSensitive: searchOptions.isCaseSensitive,
  });
};

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
};
</script>

<template>
  <div class="flex h-screen bg-gray-50">
    <!-- Sidebar -->
    <div 
      v-if="uiStore.sidebarOpen"
      class="w-64 bg-white border-r border-gray-200 flex flex-col"
    >
      <div class="p-4 border-b border-gray-200">
        <h1 class="text-lg font-semibold text-gray-900">Log Viewer</h1>
      </div>
      
      <!-- File list -->
      <div class="flex-1 overflow-y-auto p-4">
        <h2 class="text-sm font-medium text-gray-700 mb-2">Open Files</h2>
        <div v-if="fileStore.openFiles.length === 0" class="text-sm text-gray-500">
          No files open
        </div>
        <div v-else class="space-y-1">
          <div
            v-for="file in fileStore.openFiles"
            :key="file.id"
            class="flex items-center justify-between p-2 rounded cursor-pointer hover:bg-gray-100"
            :class="{ 'bg-blue-100': file.id === fileStore.currentFile?.id }"
            @click="fileStore.switchToFile(file.id)"
          >
            <span class="text-sm truncate">{{ file.name }}</span>
            <button
              @click.stop="fileStore.closeFile(file.id)"
              class="text-gray-400 hover:text-gray-600"
            >
              ×
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Main content -->
    <div class="flex-1 flex flex-col">
      <!-- Toolbar -->
      <div class="h-12 bg-white border-b border-gray-200 flex items-center px-4">
        <button
          @click="uiStore.toggleSidebar"
          class="p-1 rounded hover:bg-gray-100"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
          </svg>
        </button>
        
        <div class="flex-1 px-4">
          <span v-if="fileStore.currentFile" class="text-sm text-gray-600">
            {{ fileStore.currentFile.name }} • {{ formatFileSize(fileStore.currentFile.size) }}
          </span>
        </div>

        <button
          @click="openFile"
          class="px-3 py-1 bg-blue-500 text-white rounded text-sm hover:bg-blue-600"
        >
          Open File
        </button>
        
        <button
          @click="uiStore.toggleSearchPanel"
          class="ml-2 p-1 rounded hover:bg-gray-100"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>
        </button>
      </div>

      <!-- Content area -->
      <div class="flex-1 flex">
        <!-- File content -->
        <div class="flex-1 bg-white">
          <div v-if="!fileStore.currentFile" class="flex items-center justify-center h-full">
            <div class="text-center">
              <p class="text-gray-500 mb-4">No file selected</p>
              <button
                @click="openFile"
                class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                Open a Log File
              </button>
            </div>
          </div>
          
          <div v-else class="h-full p-4">
            <div class="bg-gray-100 rounded p-4 h-full font-mono text-sm">
              <p class="text-gray-600">File: {{ fileStore.currentFile.path }}</p>
              <p class="text-gray-600">Size: {{ formatFileSize(fileStore.currentFile.size) }}</p>
              <p class="text-gray-600 mt-2">Content preview will be implemented in Week 2-3...</p>
            </div>
          </div>
        </div>

        <!-- Search panel -->
        <div 
          v-if="uiStore.searchPanelOpen"
          class="w-80 bg-white border-l border-gray-200 flex flex-col"
        >
          <div class="p-4 border-b border-gray-200">
            <div class="flex items-center justify-between mb-3">
              <h3 class="text-sm font-medium text-gray-900">Search</h3>
              <button
                @click="uiStore.toggleSearchPanel"
                class="text-gray-400 hover:text-gray-600"
              >
                ×
              </button>
            </div>
            
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search in file..."
              class="w-full px-3 py-2 border border-gray-300 rounded text-sm"
              @keyup.enter="performSearch"
            />
            
            <div class="flex items-center mt-2 space-x-2 text-xs">
              <label class="flex items-center">
                <input
                  v-model="searchOptions.isRegex"
                  type="checkbox"
                  class="mr-1"
                />
                Regex
              </label>
              <label class="flex items-center">
                <input
                  v-model="searchOptions.isCaseSensitive"
                  type="checkbox"
                  class="mr-1"
                />
                Case sensitive
              </label>
            </div>
          </div>
          
          <div class="flex-1 p-4">
            <div v-if="searchStore.isSearching" class="text-sm text-gray-500">
              Searching...
            </div>
            <div v-else-if="searchStore.hasResults" class="text-sm text-gray-600">
              {{ searchStore.totalResults }} results
            </div>
            <div v-else-if="searchStore.hasActiveSearch" class="text-sm text-gray-500">
              No results found
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>