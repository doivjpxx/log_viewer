<template>
  <span v-if="searchTerms.length === 0">{{ text }}</span>
  <span v-else v-html="highlightedText"></span>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  text: string;
  searchTerms: string[];
  caseSensitive?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  caseSensitive: false
});

const highlightedText = computed(() => {
  if (!props.searchTerms.length || !props.text) return props.text;
  
  let result = props.text;
  
  props.searchTerms.forEach(term => {
    if (!term) return;
    
    const flags = props.caseSensitive ? 'g' : 'gi';
    const regex = new RegExp(`(${escapeRegExp(term)})`, flags);
    result = result.replace(regex, '<mark class="bg-yellow-200">$1</mark>');
  });
  
  return result;
});

function escapeRegExp(string: string): string {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
</script>