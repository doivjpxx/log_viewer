<template>
  <div class="view-mode-toggle flex items-center bg-gray-100 rounded-lg p-1">
    <button
      @click="setMode('list')"
      :class="[
        'flex items-center space-x-2 px-3 py-2 rounded-md text-sm font-medium transition-colors',
        isListMode 
          ? 'bg-white text-gray-900 shadow-sm'
          : 'text-gray-600 hover:text-gray-900'
      ]"
      :aria-pressed="isListMode"
    >
      <QueueListIcon class="h-4 w-4" />
      <span>List</span>
    </button>
    
    <button
      @click="setMode('table')"
      :class="[
        'flex items-center space-x-2 px-3 py-2 rounded-md text-sm font-medium transition-colors',
        isTableMode 
          ? 'bg-white text-gray-900 shadow-sm'
          : 'text-gray-600 hover:text-gray-900'
      ]"
      :aria-pressed="isTableMode"
    >
      <TableCellsIcon class="h-4 w-4" />
      <span>Table</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { QueueListIcon, TableCellsIcon } from '@heroicons/vue/24/outline';
import { useTableStore } from '../stores/tableStore';
import type { ViewMode } from '../types';

const tableStore = useTableStore();

const currentMode = computed(() => tableStore.viewMode);
const isListMode = computed(() => currentMode.value === 'list');
const isTableMode = computed(() => currentMode.value === 'table');

function setMode(mode: ViewMode) {
  tableStore.setViewMode(mode);
}
</script>