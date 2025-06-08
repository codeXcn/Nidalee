import { fileURLToPath, URL } from 'node:url';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vueJsx from '@vitejs/plugin-vue-jsx';
import vueDevTools from 'vite-plugin-vue-devtools';
import tailwindcss from '@tailwindcss/vite';
var host = process.env.TAURI_DEV_HOST;
// https://vite.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        vueJsx(),
        vueDevTools(),
        tailwindcss()
    ],
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url)),
        },
    },
    server: {
        port: 1422,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: 'ws',
                host: host,
                port: 1423,
            }
            : undefined,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ['**/src-tauri/**'],
        },
    },
});
