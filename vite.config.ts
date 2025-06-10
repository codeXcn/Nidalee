import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'
import tailwindcss from '@tailwindcss/vite'
import Components from 'unplugin-vue-components/vite'
import ViteAutoImport from 'unplugin-auto-import/vite'
const host = process.env.TAURI_DEV_HOST
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    vueDevTools(),
    tailwindcss(),
    ViteAutoImport({
      imports: ['vue', 'vue-router', 'pinia'],
      dts: 'types/auto-imports.d.ts',
      dirs: ['./src/hooks']
    }),
    Components({
      // 组件的有效文件扩展名。
      extensions: ['vue', 'tsx', 'jsx'],
      // 相对路径，用于搜索组件的目录。
      dirs: ['src/components'],
      // 用于转换目标的过滤器。
      include: [/\.vue$/, /\.vue\?vue/, /\.tsx$/],
      // 生成 `components.d.ts` 全局声明文件，
      dts: 'types/components.d.ts'
    })
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  server: {
    port: 1422,
    strictPort: false,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host: host,
          port: 1423
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**']
    }
  }
})
