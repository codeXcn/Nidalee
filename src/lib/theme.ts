/**
 * 颜色主题配置
 * 包含所有可用的颜色主题定义和选项
 */

// 只保留colors、radiusOptions、styles和类型定义
// 颜色选项
export const colors = [
  { name: 'zinc', label: 'Zinc', bgClass: 'bg-zinc-500' },
  { name: 'slate', label: 'Slate', bgClass: 'bg-slate-500' },
  { name: 'stone', label: 'Stone', bgClass: 'bg-stone-500' },
  { name: 'gray', label: 'Gray', bgClass: 'bg-gray-500' },
  { name: 'neutral', label: 'Neutral', bgClass: 'bg-neutral-500' },
  { name: 'red', label: 'Red', bgClass: 'bg-red-500' },
  { name: 'rose', label: 'Rose', bgClass: 'bg-rose-500' },
  { name: 'orange', label: 'Orange', bgClass: 'bg-orange-500' },
  { name: 'green', label: 'Green', bgClass: 'bg-green-500' },
  { name: 'blue', label: 'Blue', bgClass: 'bg-blue-500' },
  { name: 'yellow', label: 'Yellow', bgClass: 'bg-yellow-500' },
  { name: 'violet', label: 'Violet', bgClass: 'bg-violet-500' }
] as const

// 圆角选项
export const radiusOptions = [
  { value: 0, label: '0' },
  { value: 0.25, label: '0.25' },
  { value: 0.5, label: '0.5' },
  { value: 0.75, label: '0.75' },
  { value: 1, label: '1' }
] as const

// 风格选项
export const styles = [
  { name: 'default', label: '默认' },
  { name: 'new-york', label: 'New York' }
] as const

// 类型定义
// export type ColorThemeName = keyof typeof colorThemes
export type ColorOption = (typeof colors)[number]
export type RadiusOption = (typeof radiusOptions)[number]
export type StyleOption = (typeof styles)[number]
