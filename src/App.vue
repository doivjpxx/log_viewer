<template>
  <LogViewerLayout />
</template>

<script setup lang="ts">
import LogViewerLayout from './components/LogViewerLayout.vue'
</script>

<style>
/* Global styles */
#app {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  margin: 0;
  padding: 0;
  height: 100vh;
  overflow: hidden;
}

*, *::before, *::after {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  background: #ffffff;
}

/* Scrollbar styles */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
}

::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

/* Selection styles */
::selection {
  background-color: #3498db;
  color: white;
}

::-moz-selection {
  background-color: #3498db;
  color: white;
}

/* Focus styles */
*:focus {
  outline: 2px solid #3498db;
  outline-offset: 2px;
}

/* Button and input reset */
button, input, textarea, select {
  font-family: inherit;
}

/* Utility classes */
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.truncate {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.break-all {
  word-break: break-all;
}

/* Animation classes */
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from, .fade-leave-to {
  opacity: 0;
}

.slide-enter-active, .slide-leave-active {
  transition: transform 0.2s ease;
}

.slide-enter-from {
  transform: translateY(-10px);
}

.slide-leave-to {
  transform: translateY(-10px);
}
</style>