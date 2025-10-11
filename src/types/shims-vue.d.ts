declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}
//让TS 识别路径别名 '@/'
declare module '@/types/generated/*'
