import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface SearchResult {
  line: number;
  content: string;
  highlights: Array<{
    start: number;
    end: number;
  }>;
}

export interface SearchOptions {
  query: string;
  isRegex: boolean;
  isCaseSensitive: boolean;
  isWholeWord: boolean;
  maxResults: number;
}

export const useSearchStore = defineStore('search', () => {
  // State
  const isSearching = ref(false);
  const searchOptions = ref<SearchOptions>({
    query: '',
    isRegex: false,
    isCaseSensitive: false,
    isWholeWord: false,
    maxResults: 1000,
  });
  const searchResults = ref<SearchResult[]>([]);
  const currentResultIndex = ref(-1);
  const searchHistory = ref<string[]>([]);

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

  // Actions
  async function search(query: string, options: Partial<SearchOptions> = {}): Promise<void> {
    if (!query.trim()) {
      clearSearch();
      return;
    }

    try {
      isSearching.value = true;
      
      // Update search options
      searchOptions.value = {
        ...searchOptions.value,
        query: query.trim(),
        ...options,
      };

      // Add to history if not already present
      if (!searchHistory.value.includes(query)) {
        searchHistory.value.unshift(query);
        // Keep only last 10 searches
        if (searchHistory.value.length > 10) {
          searchHistory.value = searchHistory.value.slice(0, 10);
        }
      }

      // This will be implemented with Tauri commands
      // For now, we create a mock implementation
      searchResults.value = [];
      currentResultIndex.value = -1;
      
    } catch (error) {
      console.error('Search error:', error);
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
    
    // Getters
    hasResults,
    totalResults,
    currentResult,
    hasActiveSearch,
    
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