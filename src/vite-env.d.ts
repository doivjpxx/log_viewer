/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

// Global DOM types for Tauri environment
declare global {
  interface Window {
    __TAURI__?: any;
  }
  
  const document: Document;
  const navigator: Navigator;
}
