<template>
  <div class="demo-container p-4">
    <h2 class="text-2xl font-bold mb-4">Log Viewer Demo</h2>
    
    <div class="demo-actions mb-4 space-x-2">
      <button 
        @click="loadSampleLog"
        class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
      >
        Load Sample Log
      </button>
      
      <button 
        @click="loadStructuredLog"
        class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700"
      >
        Load Structured Log
      </button>
      
      <button 
        @click="clearData"
        class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700"
      >
        Clear Data
      </button>
    </div>
    
    <div class="demo-info mb-4 text-sm text-gray-600">
      <p>Current view mode: <strong>{{ currentViewMode }}</strong></p>
      <p v-if="tableData.length">Rows loaded: <strong>{{ tableData.length }}</strong></p>
      <p v-if="selectedRowsCount">Selected rows: <strong>{{ selectedRowsCount }}</strong></p>
    </div>
    
    <!-- Table View Demo -->
    <div v-if="isTableMode" class="table-demo border border-gray-300 rounded-lg">
      <TableViewer class="h-96" />
    </div>
    
    <!-- List View Demo -->  
    <div v-else class="list-demo border border-gray-300 rounded-lg">
      <div class="p-4 text-center text-gray-500">
        <p>List view is shown when in list mode</p>
        <p>Switch to table mode to see the table viewer</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useTableStore } from '../stores/tableStore';
import TableViewer from './TableViewer.vue';

const tableStore = useTableStore();

const currentViewMode = computed(() => tableStore.viewMode);
const isTableMode = computed(() => tableStore.isTableView);
const tableData = computed(() => tableStore.tableData);
const selectedRowsCount = computed(() => tableStore.selectedRowsCount);

function loadSampleLog() {
  const sampleLines = [
    'Starting application server...',
    'Database connection established',
    'User authentication loaded',
    'Processing 100 requests',
    'Memory usage: 1.2GB',
    'Request completed successfully',
    'Shutting down server...'
  ];
  
  const tableData = tableStore.convertToTableFormat(sampleLines);
  tableStore.setTableData(tableData);
  tableStore.setViewMode('table');
}

function loadStructuredLog() {
  const structuredLines = [
    '2024-01-01 10:00:01 INFO Starting application server on port 8080',
    '2024-01-01 10:00:02 DEBUG Database connection established successfully',
    '2024-01-01 10:00:05 INFO User authentication module loaded',
    '2024-01-01 10:00:10 WARN Cache size limit reached, purging old entries',
    '2024-01-01 10:00:15 ERROR Failed to connect to external API: timeout after 30s',
    '2024-01-01 10:00:20 INFO Retrying API connection...',
    '2024-01-01 10:00:25 INFO API connection restored successfully',
    '2024-01-01 10:01:00 INFO Processing 150 pending requests',
    '2024-01-01 10:01:05 DEBUG Memory usage: 2.1GB / 4GB allocated',
    '2024-01-01 10:01:10 WARN High CPU usage detected: 85%'
  ];
  
  const tableData = tableStore.convertToTableFormat(structuredLines);
  tableStore.setTableData(tableData);
  tableStore.autoDetectColumns(tableData);
  tableStore.setViewMode('table');
}

function clearData() {
  tableStore.clearTableData();
}
</script>

<style scoped>
.demo-container {
  max-width: 1200px;
  margin: 0 auto;
}
</style>