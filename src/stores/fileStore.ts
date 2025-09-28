import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

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

export interface FileInfo {
  path: string;
  name: string;
  size: number;
  encoding: string;
  line_count?: number;
  last_modified: string;
}

// Helper function to generate unique IDs
function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).substr(2, 9);
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
  async function openFileDialog(): Promise<string | null> {
    try {
      const result = await invoke<string | null>('open_file_dialog');
      return result;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to open file dialog';
      return null;
    }
  }

  async function openFile(filePath: string): Promise<void> {
    try {
      isLoading.value = true;
      error.value = null;
      
      // Get file info from Rust backend
      const fileInfo = await invoke<FileInfo>('get_file_info', { path: filePath });
      
      const logFile: LogFile = {
        id: generateId(),
        name: fileInfo.name,
        path: fileInfo.path,
        size: fileInfo.size,
        encoding: fileInfo.encoding,
        lines: fileInfo.line_count || 0,
        isLoading: false,
        lastModified: new Date(parseInt(fileInfo.last_modified) * 1000),
      };

      currentFile.value = logFile;
      
      // Add to open files if not already there
      if (!openFiles.value.find(f => f.path === filePath)) {
        openFiles.value.push(logFile);
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to open file';
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function loadFileChunk(fileId: string, startLine: number, endLine: number): Promise<void> {
    const file = openFiles.value.find(f => f.id === fileId);
    if (!file) {
      throw new Error('File not found');
    }

    try {
      const chunkData = await invoke<{
        start_line: number;
        end_line: number;
        content: string[];
        total_lines?: number;
      }>('read_file_chunk', {
        path: file.path,
        startLine,
        endLine,
      });

      const chunk: FileChunk = {
        startLine: chunkData.start_line,
        endLine: chunkData.end_line,
        content: chunkData.content,
      };

      updateFileChunk(fileId, chunk);
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load file chunk';
      throw err;
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
    openFileDialog,
    openFile,
    loadFileChunk,
    closeFile,
    closeAllFiles,
    switchToFile,
    updateFileChunk,
    getFileChunks,
    clearError,
  };
});