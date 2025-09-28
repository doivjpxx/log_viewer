<template>
  <div class="log-viewer-layout">
    <!-- File Toolbar -->
    <FileToolbar />

    <!-- Main Content Area -->
    <div class="main-content">
      <!-- Virtual List -->
      <VirtualList 
        v-if="currentFile"
        :container-height="containerHeight"
        :line-height="20"
        :buffer-size="20"
        class="virtual-list"
      />
      
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
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Status Bar -->
    <StatusBar />

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
import StatusBar from './StatusBar.vue'
import { useFileStore } from '../stores'

// Stores
const fileStore = useFileStore()

// Local state
const containerHeight = ref(600)
const showShortcuts = ref(false)

// Computed
const currentFile = computed(() => fileStore.currentFile)

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
  const toolbarHeight = 56 // Approximate toolbar height
  const statusBarHeight = 32 // Approximate status bar height
  const windowHeight = 800 // Fixed height for Tauri environment
  
  containerHeight.value = windowHeight - toolbarHeight - statusBarHeight - 20 // padding
}

onMounted(() => {
  updateContainerHeight()
})

onUnmounted(() => {
  // Cleanup if needed
})
</script>

<style scoped>
.log-viewer-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #ffffff;
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
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.welcome-content {
  text-align: center;
  max-width: 600px;
  padding: 40px;
}

.welcome-icon {
  font-size: 72px;
  margin-bottom: 24px;
}

.welcome-title {
  font-size: 32px;
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 16px;
}

.welcome-description {
  font-size: 18px;
  color: #7f8c8d;
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
  background: #3498db;
  color: white;
}

.welcome-btn--primary:hover {
  background: #2980b9;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(52, 152, 219, 0.3);
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
  background: rgba(255, 255, 255, 0.8);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.feature-icon {
  font-size: 18px;
}

.feature-text {
  font-size: 14px;
  color: #34495e;
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
}

.shortcuts-content {
  background: white;
  border-radius: 12px;
  padding: 24px;
  max-width: 800px;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
}

.shortcuts-content h3 {
  margin: 0 0 24px 0;
  font-size: 24px;
  color: #2c3e50;
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
  color: #34495e;
  border-bottom: 2px solid #ecf0f1;
  padding-bottom: 8px;
}

.shortcut-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid #f8f9fa;
}

.shortcut-item:last-child {
  border-bottom: none;
}

kbd {
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 4px;
  padding: 4px 8px;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 12px;
  color: #495057;
  min-width: 60px;
  text-align: center;
}

.shortcuts-footer {
  text-align: center;
  padding-top: 16px;
  border-top: 1px solid #ecf0f1;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.btn-primary {
  background: #3498db;
  color: white;
}

.btn-primary:hover {
  background: #2980b9;
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
  }

  .shortcuts-grid {
    grid-template-columns: 1fr;
  }
}
</style>