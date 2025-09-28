import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useFileStore } from './fileStore';

export interface SearchResult {
  line_number: number;
  content: string;
  highlights: Array<{
    start: number;
    end: number;
  }>;
  context_before?: string[];
  context_after?: string[];
}

export interface SearchOptions {
  query: string;
  is_regex: boolean;
  is_case_sensitive: boolean;
  is_whole_word: boolean;
  max_results: number;
  context_lines?: number;
  start_line?: number;
  end_line?: number;
}

export const useSearchStore = defineStore('search', () => {
  // State
  const isSearching = ref(false);
  const searchOptions = ref<SearchOptions>({
    query: '',
    is_regex: false,
    is_case_sensitive: false,
    is_whole_word: false,
    max_results: 1000,
    context_lines: 0,
  });
  const searchResults = ref<SearchResult[]>([]);
  const currentResultIndex = ref(-1);
  const searchHistory = ref<string[]>([]);
  const searchError = ref<string | null>(null);

  // Getters
  const hasResults = computed(() => searchResults.value.length > 0);
  const totalResults = computed(() => searchResults.value.length);
  const currentResult = computed(() => {
    if (currentResultIndex.value >= 0 && currentResultIndex.value < searchResults.value.length) {
      return searchResults.value[currentResultIndex.value];
    }
    return null;
  });
  const hasActiveSearch = computed(() => searchOptions.value.query.length > 0);
  
  const searchProgress = computed(() => {
    // For now, return simple progress based on results found
    return isSearching.value ? Math.min(90, searchResults.value.length * 2) : 100;
  });

  // Actions
  async function search(query: string, options: Partial<SearchOptions> = {}): Promise<void> {
    if (!query.trim()) {
      clearSearch();
      return;
    }

    const fileStore = useFileStore();
    if (!fileStore.currentFile) {
      searchError.value = 'No file is currently open';
      return;
    }

    try {
      isSearching.value = true;
      searchError.value = null;
      
      // Update search options
      const searchOpts = {
        ...searchOptions.value,
        query: query.trim(),
        ...options,
      };
      searchOptions.value = searchOpts;

      // Add to history if not already present
      if (!searchHistory.value.includes(query)) {
        searchHistory.value.unshift(query);
        // Keep only last 10 searches
        if (searchHistory.value.length > 10) {
          searchHistory.value = searchHistory.value.slice(0, 10);
        }
      }

      // Call Tauri backend for search
      const results = await invoke<SearchResult[]>('search_in_file', {
        path: fileStore.currentFile.path,
        options: searchOpts,
      });

      searchResults.value = results;
      currentResultIndex.value = results.length > 0 ? 0 : -1;
      
    } catch (error) {
      searchError.value = error instanceof Error ? error.message : 'Search failed';
      searchResults.value = [];
    } finally {
      isSearching.value = false;
    }
  }

  function clearSearch(): void {
    searchOptions.value.query = '';
    searchResults.value = [];
    currentResultIndex.value = -1;
    isSearching.value = false;
  }

  function nextResult(): void {
    if (searchResults.value.length === 0) return;
    
    currentResultIndex.value = (currentResultIndex.value + 1) % searchResults.value.length;
  }

  function previousResult(): void {
    if (searchResults.value.length === 0) return;
    
    currentResultIndex.value = 
      currentResultIndex.value <= 0 
        ? searchResults.value.length - 1 
        : currentResultIndex.value - 1;
  }

  function goToResult(index: number): void {
    if (index >= 0 && index < searchResults.value.length) {
      currentResultIndex.value = index;
    }
  }

  function updateSearchOptions(options: Partial<SearchOptions>): void {
    searchOptions.value = { ...searchOptions.value, ...options };
  }

  function clearHistory(): void {
    searchHistory.value = [];
  }

  return {
    // State
    isSearching,
    searchOptions,
    searchResults,
    currentResultIndex,
    searchHistory,
    searchError,
    
    // Getters
    hasResults,
    totalResults,
    currentResult,
    hasActiveSearch,
    searchProgress,
    
    // Actions
    search,
    clearSearch,
    nextResult,
    previousResult,
    goToResult,
    updateSearchOptions,
    clearHistory,
  };
});