<template>
  <div 
    ref="containerRef"
    class="virtual-list-container"
    @scroll="handleScroll"
    @keydown="handleKeyDown"
    tabindex="0"
  >
    <!-- Virtual spacer to maintain scroll height -->
    <div 
      class="virtual-spacer"
      :style="{ height: totalHeight + 'px' }"
    />
    
    <!-- Visible items container -->
    <div 
      class="visible-items"
      :style="{ 
        transform: `translateY(${startOffset}px)`,
        position: 'absolute',
        top: 0,
        left: 0,
        right: 0,
      }"
    >
      <LogLineItem
        v-for="item in visibleItems"
        :key="`${item.lineNumber}-${item.content.slice(0, 50)}`"
        :line="item"
        :line-height="lineHeight"
        :is-highlighted="isLineHighlighted(item.lineNumber)"
        :is-selected="isLineSelected(item.lineNumber)"
        :search-highlights="getSearchHighlights(item.lineNumber)"
        :line-number-width="lineNumberWidth"
        @click="handleLineClick"
        @double-click="handleLineDoubleClick"
      />
    </div>
    
    <!-- Loading overlay -->
    <div v-if="isLoading" class="loading-overlay">
      <div class="loading-spinner">
        <div class="spinner"></div>
        <div class="loading-text">Loading lines...</div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="!isLoading && totalItems === 0" class="empty-state">
      <div class="empty-icon">📄</div>
      <div class="empty-text">No content to display</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { useFileStore } from '../stores/fileStore'
import { useViewStore } from '../stores/viewStore'
import { useSearchStore } from '../stores/searchStore'
import LogLineItem from './LogLineItem.vue'

interface LogLine {
  lineNumber: number
  content: string
  level?: 'INFO' | 'WARN' | 'ERROR' | 'DEBUG'
  timestamp?: string
}

// Props
interface Props {
  containerHeight?: number
  lineHeight?: number
  bufferSize?: number
}

const props = withDefaults(defineProps<Props>(), {
  containerHeight: 600,
  lineHeight: 20,
  bufferSize: 20,
})

// Stores
const fileStore = useFileStore()
const viewStore = useViewStore()
const searchStore = useSearchStore()

// Refs
const containerRef = ref<any>()
const isLoading = ref(false)
const visibleItems = ref<LogLine[]>([])

// Computed
const totalItems = computed(() => fileStore.currentFile?.lines || 0)
const totalHeight = computed(() => totalItems.value * props.lineHeight)

const visibleCount = computed(() => 
  Math.ceil(props.containerHeight / props.lineHeight)
)

const startIndex = computed(() => {
  const scrollBasedIndex = Math.floor(viewStore.viewport.scrollTop / props.lineHeight)
  return Math.max(0, scrollBasedIndex - props.bufferSize)
})

const endIndex = computed(() => {
  const end = startIndex.value + visibleCount.value + (props.bufferSize * 2)
  return Math.min(totalItems.value, end)
})

const startOffset = computed(() => startIndex.value * props.lineHeight)

const lineNumberWidth = computed(() => {
  const digits = totalItems.value.toString().length
  return Math.max(60, digits * 8 + 16)
})

// Methods
const handleScroll = async (event: any) => {
  const target = event.target as any
  const newScrollTop = target.scrollTop
  
  viewStore.updateScrollPosition(newScrollTop)
  await loadVisibleItems()
}

const loadVisibleItems = async () => {
  if (!fileStore.currentFile || isLoading.value) return
  
  isLoading.value = true
  
  try {
    const currentFile = fileStore.currentFile
    
    // Simple approach: load first 100 lines for now
    await fileStore.loadFileChunk(currentFile.id, 0, 100)
    
    // Get chunks and create mock content
    const chunks = fileStore.getFileChunks(currentFile.id)
    const mockContent: string[] = []
    
    if (chunks.length > 0) {
      // Use actual chunk content
      const chunk = chunks[0]
      mockContent.push(...chunk.content.slice(0, 50)) // Show first 50 lines
    } else {
      // Create mock content
      for (let i = 0; i < 50; i++) {
        mockContent.push(`Line ${i + 1}: Sample log content for testing`)
      }
    }
    
    // Convert to LogLine objects
    visibleItems.value = mockContent.map((content, index) => ({
      lineNumber: index,
      content,
      level: parseLogLevel(content),
      timestamp: parseTimestamp(content),
    }))
  } catch {
    // Create fallback content for debugging
    const fallbackContent: string[] = []
    for (let i = 0; i < 20; i++) {
      fallbackContent.push(`Fallback Line ${i + 1}: Debug content`)
    }
    
    visibleItems.value = fallbackContent.map((content, index) => ({
      lineNumber: index,
      content,
      level: undefined,
      timestamp: undefined,
    }))
  } finally {
    isLoading.value = false
  }
}

const parseLogLevel = (content: string): LogLine['level'] => {
  if (content.includes('[ERROR]')) return 'ERROR'
  if (content.includes('[WARN]')) return 'WARN'
  if (content.includes('[DEBUG]')) return 'DEBUG'
  if (content.includes('[INFO]')) return 'INFO'
  return undefined
}

const parseTimestamp = (content: string): string | undefined => {
  const match = content.match(/^(\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2})/)
  return match ? match[1] : undefined
}

const handleKeyDown = (event: any) => {
  switch (event.key) {
    case 'ArrowDown':
      event.preventDefault()
      scrollBy(1)
      break
    case 'ArrowUp':
      event.preventDefault()
      scrollBy(-1)
      break
    case 'PageDown':
      event.preventDefault()
      viewStore.nextPage()
      break
    case 'PageUp':
      event.preventDefault()
      viewStore.previousPage()
      break
    case 'Home':
      if (event.ctrlKey || event.metaKey) {
        event.preventDefault()
        viewStore.scrollToTop()
      }
      break
    case 'End':
      if (event.ctrlKey || event.metaKey) {
        event.preventDefault()
        viewStore.scrollToBottom()
      }
      break
    case 'g':
      if (event.ctrlKey || event.metaKey) {
        event.preventDefault()
        viewStore.toggleJumpToLineDialog()
      }
      break
  }
}

const scrollBy = (lines: number) => {
  const newScrollTop = viewStore.viewport.scrollTop + (lines * props.lineHeight)
  const maxScrollTop = Math.max(0, totalHeight.value - props.containerHeight)
  viewStore.updateScrollPosition(Math.max(0, Math.min(newScrollTop, maxScrollTop)))
  
  if (containerRef.value) {
    containerRef.value.scrollTop = viewStore.viewport.scrollTop
  }
}

const handleLineClick = (line: LogLine, event: any) => {
  viewStore.selectLine(line.lineNumber, event.ctrlKey || event.metaKey)
}

const handleLineDoubleClick = () => {
  // Future: expand line, show details, etc.
}

const isLineHighlighted = (lineNumber: number): boolean => {
  return searchStore.currentResult?.line_number === lineNumber
}

const isLineSelected = (lineNumber: number): boolean => {
  return viewStore.navigation.selectedLines.has(lineNumber)
}

const getSearchHighlights = (lineNumber: number) => {
  const result = searchStore.searchResults.find(r => r.line_number === lineNumber)
  return result?.highlights || []
}

onMounted(() => {
  if (containerRef.value) {
    viewStore.setContainerHeight(props.containerHeight)
    loadVisibleItems()
  }
})

onUnmounted(() => {
  // Cleanup if needed
})

// Watch for viewport changes
watch(
  () => [startIndex.value, endIndex.value],
  () => {
    loadVisibleItems()
  }
)

// Watch for current file changes
watch(
  () => fileStore.currentFile,
  (newFile) => {
    if (newFile) {
      viewStore.scrollToTop()
      nextTick(() => {
        loadVisibleItems()
      })
    } else {
      visibleItems.value = []
    }
  }
)

// Watch for scroll position changes from other components
watch(
  () => viewStore.viewport.scrollTop,
  (newScrollTop) => {
    if (containerRef.value && containerRef.value.scrollTop !== newScrollTop) {
      containerRef.value.scrollTop = newScrollTop
    }
  }
)
</script>

<style scoped>
.virtual-list-container {
  position: relative;
  overflow-y: auto;
  overflow-x: hidden;
  height: 100%;
  background: #ffffff;
  outline: none;
}

.virtual-list-container:focus {
  outline: 2px solid #2196f3;
  outline-offset: -2px;
}

.virtual-spacer {
  width: 1px;
  flex-shrink: 0;
}

.visible-items {
  width: 100%;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
}

.loading-spinner {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid #f0f0f0;
  border-top: 3px solid #2196f3;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.loading-text {
  font-size: 14px;
  color: #666;
}

.empty-state {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
  color: #9e9e9e;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 16px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* Scrollbar styling */
.virtual-list-container::-webkit-scrollbar {
  width: 12px;
}

.virtual-list-container::-webkit-scrollbar-track {
  background: #f1f1f1;
}

.virtual-list-container::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 6px;
}

.virtual-list-container::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}
</style>