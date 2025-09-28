<template>
  <div id="app" :class="{ 'theme-dark': uiStore.isDarkTheme }">
    <LogViewerLayout />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import LogViewerLayout from './components/LogViewerLayout.vue'
import { useUIStore } from './stores/uiStore'
import { useFileStore } from './stores/fileStore'

const uiStore = useUIStore()
const fileStore = useFileStore()

// Global keyboard shortcuts
function handleGlobalKeydown(event: KeyboardEvent) {
  const { ctrlKey, metaKey, key } = event
  const isMac = /Mac|iPod|iPhone|iPad/.test(globalThis?.navigator?.platform ?? '')
  const cmdCtrl = isMac ? metaKey : ctrlKey

  // File operations
  if (cmdCtrl && key === 'o') {
    event.preventDefault()
    openFile()
  } else if (cmdCtrl && key === 'f') {
    event.preventDefault()
    uiStore.toggleSearchDialog()
  } else if (cmdCtrl && key === 'g') {
    event.preventDefault()
    uiStore.toggleGoToLineDialog()
  } else if (key === 'F5') {
    event.preventDefault()
    reloadCurrentFile()
  } else if (cmdCtrl && key === 'w') {
    event.preventDefault()
    closeCurrentFile()
  } else if (key === 'F1' || (cmdCtrl && key === '/')) {
    event.preventDefault()
    uiStore.toggleShortcutsHelp()
  } else if (key === 'Escape') {
    // Close any open dialogs
    uiStore.closeAllDialogs()
  }
}

async function openFile() {
  try {
    const filePath = await fileStore.openFileDialog()
    if (filePath) {
      await fileStore.openFile(filePath)
    }
  } catch {
    // Handle error silently or show user notification
  }
}

async function reloadCurrentFile() {
  if (fileStore.currentFile) {
    try {
      await fileStore.openFile(fileStore.currentFile.path)
    } catch {
      // Handle error silently or show user notification
    }
  }
}

function closeCurrentFile() {
  fileStore.closeCurrentFile()
}

onMounted(() => {
  // Initialize theme
  uiStore.initializeTheme()
  
  // Add global keyboard shortcuts
  globalThis?.document?.addEventListener('keydown', handleGlobalKeydown)
})

onUnmounted(() => {
  globalThis?.document?.removeEventListener('keydown', handleGlobalKeydown)
})
</script>

<style>
/* CSS Custom Properties for Theme System */
:root {
  /* Light Theme Colors */
  --bg-primary: #ffffff;
  --bg-secondary: #f8f9fa;
  --bg-tertiary: #e9ecef;
  --bg-accent: #3498db;
  --bg-success: #27ae60;
  --bg-warning: #f39c12;
  --bg-error: #e74c3c;
  
  --text-primary: #2c3e50;
  --text-secondary: #6c757d;
  --text-muted: #adb5bd;
  --text-inverse: #ffffff;
  --text-accent: #3498db;
  --text-success: #27ae60;
  --text-warning: #f39c12;
  --text-error: #e74c3c;
  
  --border-primary: #dee2e6;
  --border-secondary: #ced4da;
  --border-focus: #3498db;
  
  --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
  
  /* Log Level Colors */
  --log-debug: #6c757d;
  --log-info: #0dcaf0;
  --log-warn: #ffc107;
  --log-error: #dc3545;
  
  /* Scrollbar Colors */
  --scrollbar-track: #f1f1f1;
  --scrollbar-thumb: #c1c1c1;
  --scrollbar-thumb-hover: #a8a8a8;
  
  /* Selection Colors */
  --selection-bg: #3498db;
  --selection-text: #ffffff;
}

/* Dark Theme Colors */
[data-theme="dark"], .theme-dark {
  --bg-primary: #1a1a1a;
  --bg-secondary: #2d3748;
  --bg-tertiary: #4a5568;
  --bg-accent: #3498db;
  --bg-success: #38a169;
  --bg-warning: #ed8936;
  --bg-error: #e53e3e;
  
  --text-primary: #f7fafc;
  --text-secondary: #e2e8f0;
  --text-muted: #a0aec0;
  --text-inverse: #1a1a1a;
  --text-accent: #63b3ed;
  --text-success: #68d391;
  --text-warning: #fbb040;
  --text-error: #fc8181;
  
  --border-primary: #4a5568;
  --border-secondary: #2d3748;
  --border-focus: #63b3ed;
  
  --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.3);
  --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.4);
  --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.5);
  
  /* Log Level Colors - Dark Theme */
  --log-debug: #a0aec0;
  --log-info: #63b3ed;
  --log-warn: #fbb040;
  --log-error: #fc8181;
  
  /* Scrollbar Colors - Dark Theme */
  --scrollbar-track: #2d3748;
  --scrollbar-thumb: #4a5568;
  --scrollbar-thumb-hover: #718096;
  
  /* Selection Colors - Dark Theme */
  --selection-bg: #63b3ed;
  --selection-text: #1a1a1a;
}

/* Global styles */
#app {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  margin: 0;
  padding: 0;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: background-color 0.2s ease, color 0.2s ease;
}

*, *::before, *::after {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: background-color 0.2s ease, color 0.2s ease;
}

/* Scrollbar styles */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--scrollbar-track);
}

::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 4px;
  transition: background-color 0.2s ease;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--scrollbar-thumb-hover);
}

/* Selection styles */
::selection {
  background-color: var(--selection-bg);
  color: var(--selection-text);
}

::-moz-selection {
  background-color: var(--selection-bg);
  color: var(--selection-text);
}

/* Focus styles */
*:focus {
  outline: 2px solid var(--border-focus);
  outline-offset: 2px;
}

/* Button and input reset */
button, input, textarea, select {
  font-family: inherit;
  background: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-primary);
  transition: all 0.2s ease;
}

button:hover {
  background: var(--bg-secondary);
}

button:active {
  background: var(--bg-tertiary);
}

input:focus, textarea:focus, select:focus {
  border-color: var(--border-focus);
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
}

/* Utility classes */
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.truncate {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.break-all {
  word-break: break-all;
}

/* Animation classes */
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from, .fade-leave-to {
  opacity: 0;
}

.slide-enter-active, .slide-leave-active {
  transition: transform 0.2s ease;
}

.slide-enter-from {
  transform: translateY(-10px);
}

.slide-leave-to {
  transform: translateY(-10px);
}

/* Log level styling classes */
.log-debug { color: var(--log-debug); }
.log-info { color: var(--log-info); }
.log-warn { color: var(--log-warn); }
.log-error { color: var(--log-error); }

/* Responsive design utilities */
@media (max-width: 768px) {
  #app {
    font-size: 14px;
  }
}

@media (max-width: 480px) {
  #app {
    font-size: 12px;
  }
}

/* High contrast mode support */
@media (prefers-contrast: high) {
  :root {
    --border-primary: #000000;
    --text-primary: #000000;
    --bg-primary: #ffffff;
  }
  
  [data-theme="dark"], .theme-dark {
    --border-primary: #ffffff;
    --text-primary: #ffffff;
    --bg-primary: #000000;
  }
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
</style>