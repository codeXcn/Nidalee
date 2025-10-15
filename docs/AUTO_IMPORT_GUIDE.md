# 自动导入使用指南

## 📋 当前配置

根据 [`vite.config.ts`](../vite.config.ts) 的配置：

```typescript
AutoImport({
  imports: ['vue', 'vue-router', 'pinia'],
  dts: 'types/auto-imports.d.ts',
  // 使用 ** 递归扫描所有嵌套模块
  dirs: [
    './src/shared/composables/**',
    './src/shared/stores/**'
  ],
  // 扫描选项：过滤掉 index.ts 避免重复导出问题
  dirsScanOptions: {
    filePatterns: ['*.ts'],
    fileFilter: (file: string) => !file.endsWith('index.ts')
  }
})
```

### 🔍 配置说明

1. **`dirs: ['./src/shared/composables/**', './src/shared/stores/**']`**
   - 使用 `**` 通配符递归扫描所有子目录
   - **composables**：自动发现 `app/useApp.ts`, `game/useChampSelect.ts` 等
   - **stores**：自动发现 `features/gameStore.ts`, `core/connectionStore.ts` 等

2. **`dirsScanOptions.fileFilter`**
   - 过滤掉 `index.ts` 文件
   - 避免 `export * from './game-helper'` 等重复导出导致的扫描错误
   - 参考：[文档](https://unplugin.unjs.io/showcase/unplugin-auto-import#configuration)

## ✅ 可以省略的导入

### 1. Vue 3 Composition API
```typescript
// ❌ 旧写法 - 不再需要
import { ref, computed, reactive, watch, watchEffect } from 'vue'
import { onMounted, onUnmounted, provide, inject } from 'vue'

// ✅ 新写法 - 直接使用
const count = ref(0)
const doubled = computed(() => count.value * 2)
onMounted(() => {
  console.log('组件已挂载')
})
```

### 2. Vue Router
```typescript
// ❌ 旧写法
import { useRoute, useRouter } from 'vue-router'

// ✅ 新写法
const route = useRoute()
const router = useRouter()
```

### 3. Pinia
```typescript
// ❌ 旧写法
import { defineStore, storeToRefs } from 'pinia'

// ✅ 新写法
export const useMyStore = defineStore('my', () => {
  const count = ref(0)
  return { count }
})

// 使用时
const store = useMyStore()
const { count } = storeToRefs(store)
```

### 4. 自定义 Composables
```typescript
// ❌ 旧写法
import { useConnection } from '@/shared/composables/connection/useConnection'
import { useApp } from '@/shared/composables/app/useApp'

// ✅ 新写法 - 自动导入 src/shared/composables 下的所有函数
const { isConnected } = useConnection()
const { isDark } = useApp()
```

### 5. Pinia Stores ⭐ 新增
```typescript
// ❌ 旧写法
import { useGameStore } from '@/shared/stores/features/gameStore'
import { useConnectionStore } from '@/shared/stores/core/connectionStore'
import { useSettingsStore } from '@/shared/stores/ui/settingsStore'

// ✅ 新写法 - 自动导入 src/shared/stores 下的所有 stores
const gameStore = useGameStore()
const connectionStore = useConnectionStore()
const settingsStore = useSettingsStore()
```

**已支持的 Stores（10个）**：
- `useGameStore` (features)
- `useConnectionStore` (core)
- `useActivityStore` (core)
- `useDataStore` (core)
- `useAnalysisCacheStore` (features)
- `useMatchmakingStore` (features)
- `useSessionStore` (features)
- `useAutoFunctionStore` (root)
- `useSettingsStore` (ui)

## 📊 当前项目统计

**自动导入数量**：
- Vue API: 62 个 (ref, computed, watch, onMounted, ...)
- Vue Router: 5 个 (useRoute, useRouter, ...)
- Pinia: 7 个 (defineStore, storeToRefs, ...)
- Composables: ~50 个 (useApp, useConnection, ...)
- **Stores: 10 个** ⭐ (useGameStore, useConnectionStore, ...)
- **总计: ~140+ 个自动导入**

已识别可优化的文件：
- **102 个文件**包含可省略的 Vue API 导入
- **12 个文件**包含可省略的 Pinia 导入
- **10+ 个文件**包含可省略的 Store 导入 ⭐
- **3 个文件**包含可省略的 Vue Router 导入

## 🎯 优化建议

### 立即可以优化的高频文件

#### 所有 Store 文件 (src/shared/stores/\*\*/\*.ts)
```typescript
// Before
import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useMyStore = defineStore('my', () => {
  const count = ref(0)
  const doubled = computed(() => count.value * 2)
  return { count, doubled }
})

// After
export const useMyStore = defineStore('my', () => {
  const count = ref(0)
  const doubled = computed(() => count.value * 2)
  return { count, doubled }
})
```

#### 所有 Feature 组件 (src/features/\*\*/\*.vue)
```vue
<!-- Before -->
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useMyStore } from './store'

const store = useMyStore()
const { count } = storeToRefs(store)
const doubled = computed(() => count.value * 2)

onMounted(() => {
  console.log('mounted')
})
</script>

<!-- After -->
<script setup lang="ts">
import { useMyStore } from './store'

const store = useMyStore()
const { count } = storeToRefs(store)
const doubled = computed(() => count.value * 2)

onMounted(() => {
  console.log('mounted')
})
</script>
```

## 🚀 最佳实践

### 1. 保留的导入
以下导入**仍需手动导入**：
- 第三方库组件（如 `lucide-vue-next`, `vue-sonner`）
- 类型导入（`import type { ... }`）
- 项目内部模块（stores, utils 等）
- 非 `src/shared/composables` 下的 composables

### 2. TypeScript 支持
自动导入会生成 `types/auto-imports.d.ts`，确保：
- ✅ 该文件已在 `tsconfig.json` 中包含
- ✅ IDE 已识别该类型文件
- ✅ 提交代码时**不要** ignore 这个文件

### 3. ESLint 配置
如果遇到 `no-undef` 错误：
```json
// .eslintrc.js
module.exports = {
  extends: [
    './.eslintrc-auto-import.json' // 自动生成的 ESLint 配置
  ]
}
```

## 📦 扩展配置（可选）

### 添加更多预设库
参考 [unplugin-auto-import 文档](https://unplugin.unjs.io/showcase/unplugin-auto-import)：

```typescript
AutoImport({
  imports: [
    'vue',
    'vue-router',
    'pinia',
    // 添加 VueUse
    '@vueuse/core',
    // 添加自定义导入
    {
      'radash': ['isEqual', 'unique', 'groupBy']
    }
  ]
})
```

### 自动导入组件
已配置 `unplugin-vue-components`：
```typescript
Components({
  dirs: ['src/shared/components', 'src/features'],
  dts: 'types/components.d.ts'
})
```

这意味着 `shared/components` 和 `features` 下的组件**也可以自动导入**！

## ⚠️ 注意事项

1. **不要过度优化**：如果代码已经在运行，不用强制重写
2. **团队约定**：确保团队成员都了解自动导入机制
3. **性能影响**：自动导入是**编译时优化**，不会影响运行时性能
4. **调试提示**：某些 IDE 可能在自动导入时提示缺少导入，这是正常的

## 📚 参考资料

- [unplugin-auto-import 官方文档](https://unplugin.unjs.io/showcase/unplugin-auto-import)
- [Vue 3 Composition API](https://vuejs.org/guide/extras/composition-api-faq.html)
- [Pinia 官方文档](https://pinia.vuejs.org/)

