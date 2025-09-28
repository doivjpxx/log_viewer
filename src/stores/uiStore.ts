import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

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
    theme: 'auto',
    fontSize: 14,
    fontFamily: 'JetBrains Mono, "SF Mono", Monaco, Inconsolata, "Fira Code", "Fira Mono", "Droid Sans Mono", "Source Code Pro", monospace',
    lineHeight: 1.5,
    showLineNumbers: true,
    wrapLines: false,
    highlightCurrentLine: true,
    showMinimap: false,
  });
  
  const sidebarOpen = ref(false);
  const searchPanelOpen = ref(false);
  const settingsModalOpen = ref(false);
  const showShortcutsHelp = ref(false);
  const showGoToLineDialog = ref(false);
  const showSearchDialog = ref(false);
  const currentLine = ref(0);
  const selectedLines = ref<Set<number>>(new Set());

  // Window state
  const windowSize = ref({
    width: 1200,
    height: 800,
  });

  // Theme management
  const isDarkTheme = computed(() => {
    if (settings.value.theme === 'dark') return true;
    if (settings.value.theme === 'light') return false;
    // Auto theme - check system preference
    if (typeof window !== 'undefined' && window.matchMedia) {
      return window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    return false;
  });

  // Actions
  function updateSettings(newSettings: Partial<UISettings>): void {
    settings.value = { ...settings.value, ...newSettings };
    // Save to localStorage
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('logViewer:settings', JSON.stringify(settings.value));
    }
    applyTheme();
  }

  function loadSettings(): void {
    if (typeof localStorage === 'undefined') return;
    
    try {
      const saved = localStorage.getItem('logViewer:settings');
      if (saved) {
        const parsed = JSON.parse(saved);
        settings.value = { ...settings.value, ...parsed };
      }
    } catch (error) {
      // Ignore parsing errors, use defaults
    }
  }

  function initializeTheme(): void {
    loadSettings();
    applyTheme();
    
    // Listen for system theme changes
    if (typeof window !== 'undefined' && window.matchMedia) {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      mediaQuery.addEventListener('change', applyTheme);
    }
  }

  function applyTheme(): void {
    if (typeof document === 'undefined') return;
    
    const root = document.documentElement;
    const theme = isDarkTheme.value ? 'dark' : 'light';
    
    root.setAttribute('data-theme', theme);
    root.className = root.className.replace(/theme-(light|dark)/, '') + ` theme-${theme}`;
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

  function toggleShortcutsHelp(): void {
    showShortcutsHelp.value = !showShortcutsHelp.value;
  }

  function toggleSearchDialog(): void {
    showSearchDialog.value = !showSearchDialog.value;
  }

  function toggleGoToLineDialog(): void {
    showGoToLineDialog.value = !showGoToLineDialog.value;
  }

  function closeAllDialogs(): void {
    searchPanelOpen.value = false;
    settingsModalOpen.value = false;
    showShortcutsHelp.value = false;
    showGoToLineDialog.value = false;
    showSearchDialog.value = false;
  }

  function setCurrentLine(lineNumber: number): void {
    currentLine.value = lineNumber;
  }

  function selectLine(lineNumber: number): void {
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
    showShortcutsHelp,
    showGoToLineDialog,
    showSearchDialog,
    currentLine,
    selectedLines,
    windowSize,
    
    // Computed
    isDarkTheme,
    
    // Actions
    updateSettings,
    loadSettings,
    initializeTheme,
    applyTheme,
    toggleSidebar,
    toggleSearchPanel,
    toggleSettingsModal,
    toggleShortcutsHelp,
    toggleSearchDialog,
    toggleGoToLineDialog,
    closeAllDialogs,
    setCurrentLine,
    selectLine,
    selectLineRange,
    clearSelection,
    updateWindowSize,
  };
});

export { useUIStore as useUiStore };
      