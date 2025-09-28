<template>
  <div class="log-viewer-layout">
    <!-- File Toolbar -->
    <FileToolbar />

    <!-- Main Content Area -->
    <div class="main-content">
      <!-- Content when file is loaded -->
      <template v-if="currentFile">
        <!-- View Mode Toggle -->
        <div class="view-controls bg-white border-b border-gray-200 px-4 py-2">
          <ViewModeToggle />
        </div>
        
        <!-- List View -->
        <VirtualList 
          v-if="isListMode"
          :container-height="containerHeight"
          :line-height="20"
          :buffer-size="20"
          class="virtual-list flex-1"
        />
        
        <!-- Table View -->
        <TableViewer 
          v-else-if="isTableMode"
          class="flex-1"
        />
      </template>
      
      <!-- Welcome Screen -->
      <div v-else class="welcome-screen">
        <div class="welcome-content">
          <div class="welcome-icon">📄</div>
          <h2 class="welcome-title">Log File Viewer</h2>
          <p class="welcome-description">
            A high-performance log file viewer built with Tauri + Vue
          </p>
          <div class="welcome-actions">
            <button @click="openFile" class="welcome-btn welcome-btn--primary">
              📁 Open File
            </button>
            <div class="welcome-features">
              <div class="feature">
                <span class="feature-icon">⚡</span>
                <span class="feature-text">Fast virtual scrolling</span>
              </div>
              <div class="feature">
                <span class="feature-icon">🔍</span>
                <span class="feature-text">Powerful search with regex</span>
              </div>
              <div class="feature">
                <span class="feature-icon">📊</span>
                <span class="feature-text">Large file support (GB+)</span>
              </div>
              <div class="feature">
                <span class="feature-icon">🎨</span>
                <span class="feature-text">Syntax highlighting</span>
              </div>
              <div class="feature">
                <span class="feature-icon">📋</span>
                <span class="feature-text">Table & List views</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Status Bar -->
    <StatusBar />

    <!-- Advanced Search Dialog -->
    <SearchDialog />

    <!-- Filter Dialog -->
    <FilterDialog />

    <!-- Parser Configuration Dialog -->
    <ParserConfigDialog />

    <!-- Export Dialog -->
    <ExportDialog />

    <!-- Keyboard Shortcuts Help -->
    <div v-if="showShortcuts" class="shortcuts-overlay" @click="showShortcuts = false">
      <div class="shortcuts-content" @click.stop>
        <h3>Keyboard Shortcuts</h3>
        <div class="shortcuts-grid">
          <div class="shortcut-group">
            <h4>File Operations</h4>
            <div class="shortcut-item">
              <kbd>Ctrl+O</kbd>
              <span>Open file</span>
            </div>
            <div class="shortcut-item">
              <kbd>F5</kbd>
              <span>Reload file</span>
            </div>
            <div class="shortcut-item">
              <kbd>Ctrl+W</kbd>
              <span>Close file</span>
            </div>
          </div>
          
          <div class="shortcut-group">
            <h4>Navigation</h4>
            <div class="shortcut-item">
              <kbd>↑/↓</kbd>
              <span>Scroll line by line</span>
            </div>
            <div class="shortcut-item">
              <kbd>Page Up/Down</kbd>
              <span>Scroll page by page</span>
            </div>
            <div class="shortcut-item">
              <kbd>Ctrl+Home</kbd>
              <span>Go to top</span>
            </div>
            <div class="shortcut-item">
              <kbd>Ctrl+End</kbd>
              <span>Go to bottom</span>
            </div>
            <div class="shortcut-item">
              <kbd>Ctrl+G</kbd>
              <span>Go to line</span>
            </div>
          </div>
          
          <div class="shortcut-group">
            <h4>Search</h4>
            <div class="shortcut-item">
              <kbd>Ctrl+F</kbd>
              <span>Search</span>
            </div>
            <div class="shortcut-item">
              <kbd>F3</kbd>
              <span>Next result</span>
            </div>
            <div class="shortcut-item">
              <kbd>Shift+F3</kbd>
              <span>Previous result</span>
            </div>
            <div class="shortcut-item">
              <kbd>Escape</kbd>
              <span>Clear search</span>
            </div>
          </div>
          
          <div class="shortcut-group">
            <h4>Other</h4>
            <div class="shortcut-item">
              <kbd>F1</kbd>
              <span>Show shortcuts</span>
            </div>
            <div class="shortcut-item">
              <kbd>Ctrl+,</kbd>
              <span>Settings</span>
            </div>
          </div>
        </div>
        <div class="shortcuts-footer">
          <button @click="showShortcuts = false" class="btn btn-primary">
            Close
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import FileToolbar from './FileToolbar.vue'
import VirtualList from './VirtualList.vue'
import TableViewer from './TableViewer.vue'
import ViewModeToggle from './ViewModeToggle.vue'
import StatusBar from './StatusBar.vue'
import SearchDialog from './SearchDialog.vue'
import FilterDialog from './FilterDialog.vue'
import ParserConfigDialog from './ParserConfigDialog.vue'
import ExportDialog from './ExportDialog.vue'
import { useFileStore } from '../stores/fileStore'
import { useUIStore } from '../stores/uiStore'
import { useTableStore } from '../stores/tableStore'

// Stores
const fileStore = useFileStore()
const uiStore = useUIStore()
const tableStore = useTableStore()

// Local state
const containerHeight = ref(600)

// Computed
const currentFile = computed(() => fileStore.currentFile)
const isListMode = computed(() => tableStore.isListView)
const isTableMode = computed(() => tableStore.isTableView)
const showShortcuts = computed({
  get: () => uiStore.showShortcutsHelp,
  set: (value) => { if (!value) uiStore.toggleShortcutsHelp() }
})

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

function updateContainerHeight() {
  // Calculate available height for virtual list
  if (typeof globalThis !== 'undefined' && globalThis.window) {
    const toolbarHeight = 56 // Approximate toolbar height
    const statusBarHeight = 32 // Approximate status bar height
    const windowHeight = globalThis.window.innerHeight
    
    containerHeight.value = Math.max(300, windowHeight - toolbarHeight - statusBarHeight - 20)
  }
}

function handleResize() {
  updateContainerHeight()
  if (typeof globalThis !== 'undefined' && globalThis.window) {
    uiStore.updateWindowSize(globalThis.window.innerWidth, globalThis.window.innerHeight)
  }
}

onMounted(() => {
  updateContainerHeight()
  
  if (typeof globalThis !== 'undefined' && globalThis.window) {
    globalThis.window.addEventListener('resize', handleResize)
  }
})

onUnmounted(() => {
  if (typeof globalThis !== 'undefined' && globalThis.window) {
    globalThis.window.removeEventListener('resize', handleResize)
  }
})
</script>

<style scoped>
.log-viewer-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: background-color 0.2s ease, color 0.2s ease;
}

.main-content {
  flex: 1;
  min-height: 0; /* Allow flex child to shrink */
  position: relative;
}

.virtual-list {
  height: 100%;
}

/* Welcome screen */
.welcome-screen {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.welcome-content {
  text-align: center;
  max-width: 600px;
  padding: 40px;
}

.welcome-icon {
  font-size: 72px;
  margin-bottom: 24px;
  filter: grayscale(0.2);
}

.welcome-title {
  font-size: 32px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 16px;
}

.welcome-description {
  font-size: 18px;
  color: var(--text-secondary);
  margin-bottom: 32px;
  line-height: 1.6;
}

.welcome-actions {
  display: flex;
  flex-direction: column;
  gap: 24px;
  align-items: center;
}

.welcome-btn {
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 8px;
}

.welcome-btn--primary {
  background: var(--bg-accent);
  color: var(--text-inverse);
  box-shadow: var(--shadow-md);
}

.welcome-btn--primary:hover {
  background: var(--bg-accent);
  filter: brightness(1.1);
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.welcome-features {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  margin-top: 24px;
}

.feature {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-primary);
  transition: background-color 0.2s ease;
}

.feature:hover {
  background: var(--bg-tertiary);
}

.feature-icon {
  font-size: 18px;
}

.feature-text {
  font-size: 14px;
  color: var(--text-primary);
}

/* Keyboard shortcuts overlay */
.shortcuts-overlay {
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

.shortcuts-content {
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  padding: 24px;
  max-width: 800px;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: var(--shadow-lg);
}

.shortcuts-content h3 {
  margin: 0 0 24px 0;
  font-size: 24px;
  color: var(--text-primary);
  text-align: center;
}

.shortcuts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 24px;
  margin-bottom: 24px;
}

.shortcut-group h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  color: var(--text-primary);
  border-bottom: 2px solid var(--border-primary);
  padding-bottom: 8px;
}

.shortcut-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid var(--border-primary);
}

.shortcut-item:last-child {
  border-bottom: none;
}

kbd {
  background: var(--bg-secondary);
  border: 1px solid var(--border-secondary);
  border-radius: 4px;
  padding: 4px 8px;
  font-family: 'Monaco', 'Menlo', 'JetBrains Mono', monospace;
  font-size: 12px;
  color: var(--text-secondary);
  min-width: 60px;
  text-align: center;
}

.shortcuts-footer {
  text-align: center;
  padding-top: 16px;
  border-top: 1px solid var(--border-primary);
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.btn-primary {
  background: var(--bg-accent);
  color: var(--text-inverse);
}

.btn-primary:hover {
  background: var(--bg-accent);
  filter: brightness(1.1);
}

/* Responsive design */
@media (max-width: 768px) {
  .welcome-content {
    padding: 24px;
  }

  .welcome-title {
    font-size: 24px;
  }

  .welcome-description {
    font-size: 16px;
  }

  .welcome-features {
    grid-template-columns: 1fr;
  }

  .shortcuts-content {
    margin: 16px;
    padding: 16px;
    max-height: 90vh;
  }

  .shortcuts-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }

  .shortcut-group h4 {
    font-size: 14px;
  }

  .shortcut-item {
    padding: 6px 0;
  }

  kbd {
    min-width: 50px;
    font-size: 11px;
  }
}

@media (max-width: 480px) {
  .welcome-icon {
    font-size: 56px;
  }

  .welcome-title {
    font-size: 20px;
  }

  .welcome-description {
    font-size: 14px;
  }

  .shortcuts-content {
    margin: 8px;
    padding: 12px;
  }

  .shortcuts-content h3 {
    font-size: 18px;
    margin-bottom: 16px;
  }
}

/* High contrast mode support */
@media (prefers-contrast: high) {
  .feature {
    border: 2px solid var(--border-primary);
  }
  
  .shortcuts-content {
    border: 2px solid var(--border-primary);
  }
  
  kbd {
    border: 2px solid var(--border-secondary);
  }
}
</style>