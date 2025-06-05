/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

// Tauri API 类型声明
declare global {
  const __TAURI_PLATFORM__: string
  const __TAURI_ARCH__: string
}
