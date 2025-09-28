import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface ViewportState {
  startLine: number;
  endLine: number;
  scrollTop: number;
  containerHeight: number;
}

export interface NavigationState {
  currentLine: number;
  selectedLines: Set<number>;
  jumpToLineDialogOpen: boolean;
}

export const useViewStore = defineStore('view', () => {
  // Viewport state
  const viewport = ref<ViewportState>({
    startLine: 0,
    endLine: 0,
    scrollTop: 0,
    containerHeight: 600,
  });

  // Navigation state
  const navigation = ref<NavigationState>({
    currentLine: 0,
    selectedLines: new Set<number>(),
    jumpToLineDialogOpen: false,
  });

  // Virtual list settings
  const lineHeight = ref(20);
  const bufferSize = ref(20);
  const containerPadding = ref(10);

  // Getters
  const visibleLineCount = computed(() => 
    Math.ceil(viewport.value.containerHeight / lineHeight.value)
  );

  const startIndex = computed(() => 
    Math.max(0, Math.floor(viewport.value.scrollTop / lineHeight.value) - bufferSize.value)
  );

  const endIndex = computed(() => 
    startIndex.value + visibleLineCount.value + (bufferSize.value * 2)
  );

  const totalHeight = computed(() => {
    const fileStore = useFileStore();
    if (!fileStore.currentFile) return 0;
    return fileStore.currentFile.lines * lineHeight.value;
  });

  // Actions
  function setViewport(startLine: number, endLine: number) {
    viewport.value.startLine = startLine;
    viewport.value.endLine = endLine;
  }

  function updateScrollPosition(scrollTop: number) {
    viewport.value.scrollTop = scrollTop;
  }

  function setContainerHeight(height: number) {
    viewport.value.containerHeight = height;
  }

  function jumpToLine(lineNumber: number) {
    navigation.value.currentLine = lineNumber;
    const targetScrollTop = (lineNumber - 1) * lineHeight.value;
    viewport.value.scrollTop = targetScrollTop;
  }

  function selectLine(lineNumber: number, multiSelect = false) {
    if (!multiSelect) {
      navigation.value.selectedLines.clear();
    }
    navigation.value.selectedLines.add(lineNumber);
  }

  function clearSelection() {
    navigation.value.selectedLines.clear();
  }

  function toggleJumpToLineDialog() {
    navigation.value.jumpToLineDialogOpen = !navigation.value.jumpToLineDialogOpen;
  }

  function setLineHeight(height: number) {
    lineHeight.value = height;
  }

  function nextPage() {
    const scrollAmount = viewport.value.containerHeight * 0.9; // 90% of container height
    viewport.value.scrollTop += scrollAmount;
  }

  function previousPage() {
    const scrollAmount = viewport.value.containerHeight * 0.9;
    viewport.value.scrollTop = Math.max(0, viewport.value.scrollTop - scrollAmount);
  }

  function scrollToTop() {
    viewport.value.scrollTop = 0;
  }

  function scrollToBottom() {
    const fileStore = useFileStore();
    if (fileStore.currentFile) {
      viewport.value.scrollTop = fileStore.currentFile.lines * lineHeight.value;
    }
  }

  return {
    // State
    viewport,
    navigation,
    lineHeight,
    bufferSize,
    containerPadding,

    // Getters
    visibleLineCount,
    startIndex,
    endIndex,
    totalHeight,

    // Actions
    setViewport,
    updateScrollPosition,
    setContainerHeight,
    jumpToLine,
    selectLine,
    clearSelection,
    toggleJumpToLineDialog,
    setLineHeight,
    nextPage,
    previousPage,
    scrollToTop,
    scrollToBottom,
  };
});

// Import useFileStore after definition to avoid circular dependency
import { useFileStore } from './fileStore';