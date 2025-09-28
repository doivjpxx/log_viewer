import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface TextFilterPattern {
  pattern: string;
  type: 'contains' | 'regex' | 'exact';
}

export interface TimeRange {
  start: string;
  end: string;
}

export interface FilterOptions {
  logLevels?: string[];
  includePatterns?: TextFilterPattern[];
  excludePatterns?: TextFilterPattern[];
  timeRange?: TimeRange | null;
}

export interface FilterResult {
  lineNumber: number;
  content: string;
  matchedFilters: string[];
}

export const useFilterStore = defineStore('filter', () => {
  // State
  const currentFilters = ref<FilterOptions | null>(null);
  const filteredLines = ref<FilterResult[]>([]);
  const isFiltering = ref(false);
  const filterError = ref<string | null>(null);
  const filterPresets = ref<Record<string, FilterOptions>>({});

  // Getters
  const hasActiveFilters = computed(() => {
    if (!currentFilters.value) return false;
    
    return (
      (currentFilters.value.logLevels && currentFilters.value.logLevels.length > 0) ||
      (currentFilters.value.includePatterns && currentFilters.value.includePatterns.some(p => p.pattern.trim())) ||
      (currentFilters.value.excludePatterns && currentFilters.value.excludePatterns.some(p => p.pattern.trim())) ||
      (currentFilters.value.timeRange && (currentFilters.value.timeRange.start || currentFilters.value.timeRange.end))
    );
  });

  const filteredLineCount = computed(() => filteredLines.value.length);
  
  const filteredLineNumbers = computed(() => 
    filteredLines.value.map(line => line.lineNumber)
  );

  // Actions
  async function applyFilters(filters: FilterOptions): Promise<void> {
    try {
      isFiltering.value = true;
      filterError.value = null;
      currentFilters.value = filters;

      // For now, we'll implement basic client-side filtering
      // In a real implementation, this would call the Rust backend
      const results = await filterLinesClientSide(filters);
      filteredLines.value = results;

    } catch (error) {
      filterError.value = error instanceof Error ? error.message : 'Filtering failed';
      throw error;
    } finally {
      isFiltering.value = false;
    }
  }

  async function clearFilters(): Promise<void> {
    currentFilters.value = null;
    filteredLines.value = [];
    filterError.value = null;
  }

  function saveFilterPreset(name: string, filters: FilterOptions): void {
    filterPresets.value[name] = { ...filters };
    // In a real app, save to localStorage or backend
    if (typeof globalThis !== 'undefined' && globalThis.localStorage) {
      globalThis.localStorage.setItem('log-viewer-filter-presets', JSON.stringify(filterPresets.value));
    }
  }

  function loadFilterPreset(name: string): FilterOptions | null {
    return filterPresets.value[name] || null;
  }

  function deleteFilterPreset(name: string): void {
    delete filterPresets.value[name];
    if (typeof globalThis !== 'undefined' && globalThis.localStorage) {
      globalThis.localStorage.setItem('log-viewer-filter-presets', JSON.stringify(filterPresets.value));
    }
  }

  // Helper function for client-side filtering (temporary implementation)
  async function filterLinesClientSide(filters: FilterOptions): Promise<FilterResult[]> {
    // This is a placeholder implementation
    // In the real app, this would be handled by the Rust backend
    
    const results: FilterResult[] = [];
    
    // Simulate some filtered results
    for (let i = 1; i <= 100; i++) {
      const content = `[INFO] Sample log line ${i} with some content`;
      const matchedFilters: string[] = [];
      
      // Simple log level matching
      if (filters.logLevels && filters.logLevels.length > 0) {
        const hasLevel = filters.logLevels.some(level => content.includes(`[${level}]`));
        if (hasLevel) {
          matchedFilters.push('logLevel');
        }
      } else {
        // If no level filters specified, include all
        matchedFilters.push('logLevel');
      }
      
      // Simple text filtering
      let includeMatch = true;
      let excludeMatch = false;
      
      if (filters.includePatterns && filters.includePatterns.length > 0) {
        includeMatch = filters.includePatterns.some(pattern => {
          if (!pattern.pattern.trim()) return true;
          
          switch (pattern.type) {
            case 'contains':
              return content.toLowerCase().includes(pattern.pattern.toLowerCase());
            case 'exact':
              return content === pattern.pattern;
            case 'regex':
              try {
                return new RegExp(pattern.pattern, 'i').test(content);
              } catch {
                return false;
              }
            default:
              return false;
          }
        });
      }
      
      if (filters.excludePatterns && filters.excludePatterns.length > 0) {
        excludeMatch = filters.excludePatterns.some(pattern => {
          if (!pattern.pattern.trim()) return false;
          
          switch (pattern.type) {
            case 'contains':
              return content.toLowerCase().includes(pattern.pattern.toLowerCase());
            case 'exact':
              return content === pattern.pattern;
            case 'regex':
              try {
                return new RegExp(pattern.pattern, 'i').test(content);
              } catch {
                return false;
              }
            default:
              return false;
          }
        });
      }
      
      // Include line if it matches include filters and doesn't match exclude filters
      if (matchedFilters.length > 0 && includeMatch && !excludeMatch) {
        results.push({
          lineNumber: i,
          content,
          matchedFilters
        });
      }
    }
    
    return results;
  }

  // Initialize presets from localStorage
  function loadPresets(): void {
    try {
      if (typeof globalThis !== 'undefined' && globalThis.localStorage) {
        const saved = globalThis.localStorage.getItem('log-viewer-filter-presets');
        if (saved) {
          filterPresets.value = JSON.parse(saved);
        }
      }
    } catch {
      // Failed to load presets, continue with empty presets
    }
  }

  // Load presets on store creation
  loadPresets();

  return {
    // State
    currentFilters,
    filteredLines,
    isFiltering,
    filterError,
    filterPresets,
    
    // Getters
    hasActiveFilters,
    filteredLineCount,
    filteredLineNumbers,
    
    // Actions
    applyFilters,
    clearFilters,
    saveFilterPreset,
    loadFilterPreset,
    deleteFilterPreset,
  };
});