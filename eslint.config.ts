import { globalIgnores } from 'eslint/config'
import { defineConfigWithVueTs, vueTsConfigs } from '@vue/eslint-config-typescript'
import pluginVue from 'eslint-plugin-vue'
import pluginOxlint from 'eslint-plugin-oxlint'
import skipFormatting from '@vue/eslint-config-prettier/skip-formatting'

// To allow more languages other than `ts` in `.vue` files, uncomment the following lines:
// import { configureVueProject } from '@vue/eslint-config-typescript'
// configureVueProject({ scriptLangs: ['ts', 'tsx'] })
// More info at https://github.com/vuejs/eslint-config-typescript/#advanced-setup

export default defineConfigWithVueTs(
  {
    name: 'app/files-to-lint',
    files: ['**/*.{ts,mts,tsx,vue}']
  },

  globalIgnores(['**/dist/**', '**/dist-ssr/**', '**/coverage/**']),

  pluginVue.configs['flat/essential'],
  vueTsConfigs.recommended,
  ...pluginOxlint.configs['flat/recommended'],
  skipFormatting,
  {
    rules: {
      '@typescript-eslint/no-empty-object-type': 0,
      '@typescript-eslint/no-unused-vars': 1, //允许未使用的变量
      '@typescript-eslint/no-explicit-any': 0, //允许使用any类型
      // 要求组件名称始终为 “-” 链接的单词
      'vue/multi-word-component-names': 'off',
      // 关闭 index.html 文件报 clear 错误
      'vue/comment-directive': 'off',

      // 关闭对 defineProps 的有效性检查
      'vue/valid-define-props': 'off',

      // 允许在一个文件中定义多个组件
      'vue/one-component-per-file': 'off',

      // 关闭 Prop 类型要求的警告
      'vue/require-prop-types': 'off',
      // 关闭属性顺序要求
      'vue/attributes-order': 'off',
      // 关闭对默认 Prop 的要求
      'vue/require-default-prop': 'off',

      // 关闭连字符命名检验
      'vue/attribute-hyphenation': 'off',

      // 关闭自闭合标签的要求
      'vue/html-self-closing': 'off',

      // 禁止在关闭的括号前有换行
      'vue/html-closing-bracket-newline': 'off',
      // 允许使用 v-html 指令
      'vue/no-v-html': 'off'
    }
  }
)
