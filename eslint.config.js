import eslint from '@eslint/js'
import tseslint from '@typescript-eslint/eslint-plugin'
import tsParser from '@typescript-eslint/parser'
import eslintConfigPrettier from 'eslint-config-prettier'
import vue from 'eslint-plugin-vue'
import globals from 'globals'

export default [
  {
    ignores: ['dist/**', 'node_modules/**']
  },
  eslint.configs.recommended,
  {
    files: ['**/*.js', '**/*.ts', '**/*.vue'],
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.es2024,
        ...globals.node
      },
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        ecmaFeatures: {
          jsx: true
        }
      }
    }
  },
  {
    files: ['**/*.vue'],
    plugins: {
      vue,
      '@typescript-eslint': tseslint
    },
    languageOptions: {
      parser: vue.parser,
      parserOptions: {
        parser: tsParser,
        ecmaVersion: 'latest',
        sourceType: 'module',
        ecmaFeatures: {
          jsx: true
        }
      }
    },
    rules: {
      'vue/script-setup-uses-vars': 'error',
      'vue/no-unused-vars': 'error',
      'vue/no-v-html': 'error',
      'vue/component-name-in-template-casing': ['error', 'PascalCase'],
      'vue/component-tags-order': ['error', {
        order: ['script', 'template', 'style']
      }],
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-unused-vars': ['error', {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_'
      }],
      '@typescript-eslint/consistent-type-imports': ['error', {
        prefer: 'type-imports'
      }],
      'no-console': ['warn', { 
        allow: ['warn', 'error'] 
      }],
      'quotes': ['error', 'single'],
      'semi': ['error', 'never']
    }
  },
  eslintConfigPrettier
] 