import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface LogFile {
  id: string;
  name: string;
  path: string;
  size: number;
  encoding: string;
  lines: number;
  isLoading: boolean;
  lastModified: Date;
}

export interface FileChunk {
  startLine: number;
  endLine: number;
  content: string[];
}

export const useFileStore = defineStore('file', () => {
  // State
  const currentFile = ref<LogFile | null>(null);
  const openFiles = ref<LogFile[]>([]);
  const fileChunks = ref<Map<string, FileChunk[]>>(new Map());
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const hasOpenFile = computed(() => currentFile.value !== null);
  const currentFileName = computed(() => currentFile.value?.name || '');
  const currentFileSize = computed(() => {
    if (!currentFile.value) return 0;
    return currentFile.value.size;
  });
  const currentFileLines = computed(() => {
    if (!currentFile.value) return 0;
    return currentFile.value.lines;
  });

  // Actions
  async function openFile(filePath: string): Promise<void> {
    try {
      isLoading.value = true;
      error.value = null;
      
      // This will be implemented with Tauri commands
      // For now, we create a mock implementation
      const fileInfo: LogFile = {
        id: crypto.randomUUID(),
        name: filePath.split('/').pop() || '',
        path: filePath,
        size: 0,
        encoding: 'UTF-8',
        lines: 0,
        isLoading: false,
        lastModified: new Date(),
      };

      currentFile.value = fileInfo;
      
      // Add to open files if not already there
      if (!openFiles.value.find(f => f.path === filePath)) {
        openFiles.value.push(fileInfo);
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to open file';
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  function closeFile(fileId: string): void {
    openFiles.value = openFiles.value.filter(f => f.id !== fileId);
    fileChunks.value.delete(fileId);
    
    if (currentFile.value?.id === fileId) {
      currentFile.value = openFiles.value[0] || null;
    }
  }

  function closeAllFiles(): void {
    openFiles.value = [];
    fileChunks.value.clear();
    currentFile.value = null;
  }

  function switchToFile(fileId: string): void {
    const file = openFiles.value.find(f => f.id === fileId);
    if (file) {
      currentFile.value = file;
    }
  }

  function updateFileChunk(fileId: string, chunk: FileChunk): void {
    const chunks = fileChunks.value.get(fileId) || [];
    
    // Find existing chunk or add new one
    const existingIndex = chunks.findIndex(
      c => c.startLine === chunk.startLine && c.endLine === chunk.endLine
    );
    
    if (existingIndex >= 0) {
      chunks[existingIndex] = chunk;
    } else {
      chunks.push(chunk);
      chunks.sort((a, b) => a.startLine - b.startLine);
    }
    
    fileChunks.value.set(fileId, chunks);
  }

  function getFileChunks(fileId: string): FileChunk[] {
    return fileChunks.value.get(fileId) || [];
  }

  function clearError(): void {
    error.value = null;
  }

  return {
    // State
    currentFile,
    openFiles,
    isLoading,
    error,
    
    // Getters
    hasOpenFile,
    currentFileName,
    currentFileSize,
    currentFileLines,
    
    // Actions
    openFile,
    closeFile,
    closeAllFiles,
    switchToFile,
    updateFileChunk,
    getFileChunks,
    clearError,
  };
});