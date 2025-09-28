import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface UISettings {
  theme: 'light' | 'dark' | 'auto';
  fontSize: number;
  fontFamily: string;
  lineHeight: number;
  showLineNumbers: boolean;
  wrapLines: boolean;
  highlightCurrentLine: boolean;
  showMinimap: boolean;
}

export const useUIStore = defineStore('ui', () => {
  // State
  const settings = ref<UISettings>({
    theme: 'light',
    fontSize: 14,
    fontFamily: 'JetBrains Mono',
    lineHeight: 1.5,
    showLineNumbers: true,
    wrapLines: false,
    highlightCurrentLine: true,
    showMinimap: false,
  });
  
  const sidebarOpen = ref(false);
  const searchPanelOpen = ref(false);
  const settingsModalOpen = ref(false);
  const currentLine = ref(0);
  const selectedLines = ref<Set<number>>(new Set());

  // Window state
  const windowSize = ref({
    width: 1200,
    height: 800,
  });

  // Actions
  function updateSettings(newSettings: Partial<UISettings>): void {
    settings.value = { ...settings.value, ...newSettings };
    // Save to localStorage
    localStorage.setItem('logViewer:settings', JSON.stringify(settings.value));
  }

  function loadSettings(): void {
    try {
      const saved = localStorage.getItem('logViewer:settings');
      if (saved) {
        const parsed = JSON.parse(saved);
        settings.value = { ...settings.value, ...parsed };
      }
    } catch (error) {
      console.warn('Failed to load settings:', error);
    }
  }

  function toggleSidebar(): void {
    sidebarOpen.value = !sidebarOpen.value;
  }

  function toggleSearchPanel(): void {
    searchPanelOpen.value = !searchPanelOpen.value;
  }

  function toggleSettingsModal(): void {
    settingsModalOpen.value = !settingsModalOpen.value;
  }

  function setCurrentLine(lineNumber: number): void {
    currentLine.value = lineNumber;
  }

  function selectLine(lineNumber: number, isMultiSelect = false): void {
    if (!isMultiSelect) {
      selectedLines.value.clear();
    }
    
    if (selectedLines.value.has(lineNumber)) {
      selectedLines.value.delete(lineNumber);
    } else {
      selectedLines.value.add(lineNumber);
    }
  }

  function selectLineRange(startLine: number, endLine: number): void {
    selectedLines.value.clear();
    for (let i = Math.min(startLine, endLine); i <= Math.max(startLine, endLine); i++) {
      selectedLines.value.add(i);
    }
  }

  function clearSelection(): void {
    selectedLines.value.clear();
  }

  function updateWindowSize(width: number, height: number): void {
    windowSize.value = { width, height };
  }

  // Initialize settings on store creation
  loadSettings();

  return {
    // State
    settings,
    sidebarOpen,
    searchPanelOpen,
    settingsModalOpen,
    currentLine,
    selectedLines,
    windowSize,
    
    // Actions
    updateSettings,
    loadSettings,
    toggleSidebar,
    toggleSearchPanel,
    toggleSettingsModal,
    setCurrentLine,
    selectLine,
    selectLineRange,
    clearSelection,
    updateWindowSize,
  };
});