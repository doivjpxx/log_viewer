<template>
  <div class="table-viewer h-full flex flex-col">
    <!-- Table Toolbar -->
    <div class="table-toolbar bg-white border-b border-gray-200 px-4 py-2 flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <!-- Row count info -->
        <div class="text-sm text-gray-600">
          {{ tableData.length.toLocaleString() }} rows
          <span v-if="selectedRowsCount > 0" class="ml-2">
            ({{ selectedRowsCount }} selected)
          </span>
        </div>
        
        <!-- Column visibility toggle -->
        <div class="relative">
          <button
            @click="showColumnMenu = !showColumnMenu"
            class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 border border-gray-300 rounded flex items-center space-x-1"
          >
            <ViewColumnsIcon class="h-4 w-4" />
            <span>Columns</span>
          </button>
          
          <!-- Column menu dropdown -->
          <div
            v-if="showColumnMenu"
            class="absolute top-full left-0 mt-1 w-64 bg-white border border-gray-200 rounded-md shadow-lg z-10"
            @click.stop
          >
            <div class="p-3">
              <div class="text-sm font-medium mb-2">Show/Hide Columns</div>
              <div class="space-y-1 max-h-48 overflow-y-auto">
                <label
                  v-for="column in allColumns"
                  :key="column.field"
                  class="flex items-center space-x-2 text-sm cursor-pointer hover:bg-gray-50 p-1 rounded"
                >
                  <input
                    type="checkbox"
                    :checked="visibleColumns.has(column.field)"
                    @change="toggleColumnVisibility(column.field)"
                    class="rounded border-gray-300"
                  />
                  <span>{{ column.title }}</span>
                </label>
              </div>
            </div>
          </div>
        </div>
        
        <!-- Selection actions -->
        <div v-if="selectedRowsCount > 0" class="flex items-center space-x-2">
          <button
            @click="clearSelection"
            class="px-3 py-1 text-sm bg-red-100 hover:bg-red-200 text-red-700 border border-red-300 rounded"
          >
            Clear Selection
          </button>
          <button
            @click="copySelectedRows"
            class="px-3 py-1 text-sm bg-blue-100 hover:bg-blue-200 text-blue-700 border border-blue-300 rounded flex items-center space-x-1"
          >
            <ClipboardDocumentIcon class="h-4 w-4" />
            <span>Copy</span>
          </button>
        </div>
      </div>
      
      <div class="flex items-center space-x-2">
        <!-- Refresh button -->
        <button
          @click="refreshTable"
          :disabled="isLoading"
          class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 border border-gray-300 rounded flex items-center space-x-1 disabled:opacity-50"
        >
          <ArrowPathIcon class="h-4 w-4" :class="{ 'animate-spin': isLoading }" />
          <span>Refresh</span>
        </button>
      </div>
    </div>

    <!-- Table Container -->
    <div class="table-container flex-1 overflow-hidden">
      <vxe-table
        ref="tableRef"
        :data="tableData"
        :loading="isLoading"
        height="100%"
        border
        stripe
        resizable
        show-header-overflow
        show-overflow="tooltip"
        :checkbox-config="{ checkField: 'checked', trigger: 'row' }"
        @checkbox-change="onSelectionChange"
        @checkbox-all="onSelectAll"
        class="log-table"
      >
        <!-- Selection column -->
        <vxe-column type="checkbox" width="50" fixed="left"></vxe-column>
        
        <!-- Dynamic columns based on visibility -->
        <template v-for="column in visibleColumnsConfig" :key="column.field">
          <vxe-column
            :field="column.field"
            :title="column.title"
            :width="column.width"
            :min-width="column.minWidth || 100"
            :sortable="column.sortable"
            :resizable="column.resizable"
            :fixed="column.field === 'lineNumber' ? 'left' : undefined"
            show-overflow="tooltip"
          >
            <template #default="{ row, column }">
              <div class="cell-content">
                <template v-if="column.field === 'level'">
                  <span
                    v-if="row.level"
                    :style="{ color: getLogLevelColor(row.level) }"
                    class="font-medium"
                  >
                    {{ row.level }}
                  </span>
                  <span v-else class="text-gray-400">-</span>
                </template>
                
                <template v-else-if="column.field === 'timestamp'">
                  <span v-if="row.timestamp" class="font-mono text-xs">
                    {{ formatTimestamp(row.timestamp) }}
                  </span>
                  <span v-else class="text-gray-400">-</span>
                </template>
                
                <template v-else-if="column.field === 'lineNumber'">
                  <span class="font-mono text-sm text-gray-500">
                    {{ row.lineNumber }}
                  </span>
                </template>
                
                <template v-else-if="column.field === 'message'">
                  <div class="message-cell font-mono text-sm whitespace-pre-wrap break-words">
                    <HighlightedText
                      :text="row.message || row.rawContent"
                      :search-terms="searchHighlights"
                      :case-sensitive="searchOptions.is_case_sensitive"
                    />
                  </div>
                </template>
                
                <template v-else>
                  <span v-if="row[column.field] !== undefined && row[column.field] !== null">
                    {{ formatCellValue(row[column.field]) }}
                  </span>
                  <span v-else class="text-gray-400">-</span>
                </template>
              </div>
            </template>
          </vxe-column>
        </template>
      </vxe-table>
    </div>

    <!-- Click outside to close column menu -->
    <div
      v-if="showColumnMenu"
      class="fixed inset-0 z-0"
      @click="showColumnMenu = false"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { 
  ViewColumnsIcon, 
  ClipboardDocumentIcon, 
  ArrowPathIcon 
} from '@heroicons/vue/24/outline';
import { useTableStore } from '../stores/tableStore';
import { useFileStore } from '../stores/fileStore';
import { useSearchStore } from '../stores/searchStore';
import type { LogLevel } from '../types';
import HighlightedText from './HighlightedText.vue';

// Stores
const tableStore = useTableStore();
const fileStore = useFileStore();
const searchStore = useSearchStore();

// Local state
const tableRef = ref();
const showColumnMenu = ref(false);
const visibleColumns = ref<Set<string>>(new Set());

// Computed properties
const tableData = computed(() => tableStore.tableData);
const allColumns = computed(() => tableStore.allColumns);
const isLoading = computed(() => tableStore.isLoading || fileStore.isLoading);
const selectedRowsCount = computed(() => tableStore.selectedRowsCount);
const searchHighlights = computed(() => searchStore.searchOptions.query ? [searchStore.searchOptions.query] : []);
const searchOptions = computed(() => searchStore.searchOptions);

const visibleColumnsConfig = computed(() => {
  return allColumns.value.filter(column => visibleColumns.value.has(column.field));
});

// Methods
function initializeVisibleColumns() {
  // Show all default columns initially
  allColumns.value.forEach(column => {
    visibleColumns.value.add(column.field);
  });
}

function toggleColumnVisibility(field: string) {
  if (visibleColumns.value.has(field)) {
    visibleColumns.value.delete(field);
  } else {
    visibleColumns.value.add(field);
  }
}

function onSelectionChange(params: any) {
  const { row, checked } = params;
  if (checked) {
    tableStore.selectRow(row.id);
  } else {
    tableStore.deselectRow(row.id);
  }
}

function onSelectAll(params: any) {
  const { checked } = params;
  if (checked) {
    tableStore.selectAllRows();
  } else {
    tableStore.clearSelection();
  }
}

function clearSelection() {
  tableStore.clearSelection();
  if (tableRef.value) {
    tableRef.value.clearCheckboxRow();
  }
}

async function copySelectedRows() {
  const selectedIds = Array.from(tableStore.selectedRows);
  const selectedData = tableData.value.filter(row => selectedIds.includes(row.id));
  
  const text = selectedData.map(row => row.rawContent).join('\n');
  
  try {
    if (globalThis.navigator && globalThis.navigator.clipboard) {
      await globalThis.navigator.clipboard.writeText(text);
    }
    // TODO: Show success toast
  } catch (error) {
    // eslint-disable-next-line no-console
    console.error('Failed to copy to clipboard:', error);
    // TODO: Show error toast
  }
}

function refreshTable() {
  // Refresh from current file data
  const currentFile = fileStore.currentFile;
  if (currentFile) {
    loadTableData();
  }
}

function formatTimestamp(timestamp: string): string {
  try {
    return new Date(timestamp).toLocaleString();
  } catch {
    return timestamp;
  }
}

function formatCellValue(value: unknown): string {
  if (value === null || value === undefined) return '-';
  if (typeof value === 'object') return JSON.stringify(value);
  return String(value);
}

function getLogLevelColor(level: LogLevel): string {
  return tableStore.getLogLevelColor(level);
}

async function loadTableData() {
  tableStore.setLoading(true);
  
  try {
    const currentFile = fileStore.currentFile;
    if (!currentFile) {
      tableStore.clearTableData();
      return;
    }

    // Load file chunks and convert to table format
    const chunks = fileStore.getFileChunks(currentFile.id || '');
    const allLines: string[] = [];
    
    for (const chunk of chunks) {
      allLines.push(...chunk.content);
    }
    
    // If no chunks loaded, load first chunk
    if (allLines.length === 0 && currentFile.lines > 0) {
      await fileStore.loadFileChunk(currentFile.id || '', 0, Math.min(99, currentFile.lines - 1));
      const newChunks = fileStore.getFileChunks(currentFile.id || '');
      for (const chunk of newChunks) {
        allLines.push(...chunk.content);
      }
    }
    
    const tableData = tableStore.convertToTableFormat(allLines);
    tableStore.setTableData(tableData);
    
    // Auto-detect columns if needed
    tableStore.autoDetectColumns(tableData);
    
    // Update visible columns
    initializeVisibleColumns();
    
  } catch (error) {
    globalThis.console?.error('Failed to load table data:', error);
  } finally {
    tableStore.setLoading(false);
  }
}

// Watch for file changes
watch(() => fileStore.currentFile, () => {
  loadTableData();
});

watch(() => tableStore.customColumns, () => {
  // Update visible columns when custom columns change
  tableStore.customColumns.forEach(column => {
    visibleColumns.value.add(column.field);
  });
});

onMounted(() => {
  initializeVisibleColumns();
  loadTableData();
});
</script>

<style scoped>
.table-viewer {
  background-color: white;
}

.message-cell {
  max-height: 200px;
  overflow-y: auto;
  line-height: 1.4;
}

.cell-content {
  padding-top: 0.25rem;
  padding-bottom: 0.25rem;
}

:deep(.vxe-table .vxe-body--row.row--stripe) {
  background-color: #fafafa;
}

:deep(.vxe-table .vxe-body--row:hover) {
  background-color: #f0f9ff;
}

:deep(.vxe-table .vxe-cell) {
  padding: 4px 8px;
}

:deep(.vxe-table .vxe-header--column) {
  background-color: #f9fafb;
  font-weight: 500;
  color: #374151;
}

:deep(.vxe-table .vxe-table--border-line) {
  border-color: #e5e7eb;
}

:deep(.log-table .vxe-table--loading .vxe-table--spinner) {
  color: #3b82f6;
}
</style>