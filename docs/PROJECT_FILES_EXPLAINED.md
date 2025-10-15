# 项目文件说明

## 📁 配置文件详解

### Node.js 相关

#### ✅ `.nvmrc` (保留)
```
20.17.0
```
**用途**：
- Node 版本管理器（nvm/fnm）的配置文件
- 团队成员运行 `nvm use` 时自动切换到指定版本
- CI/CD 可以读取这个文件来设置 Node 版本

**最佳实践**：
```bash
# 使用指定的 Node 版本
nvm use

# 或者
fnm use
```

---

#### ✅ `package.json` (保留)
```json
{
  "engines": {
    "node": ">=20.0.0",
    "pnpm": ">=10.0.0"
  }
}
```
**用途**：
- 项目依赖管理
- 脚本命令定义
- **Node 版本要求**（会在 `pnpm install` 时检查）

---

### TypeScript 相关

#### ✅ `tsconfig.json` (保留)
**用途**：TypeScript 主配置文件
**内容**：
- 编译选项
- 路径别名（`@/*` → `./src/*`）
- 包含的文件（`src/**/*`, `types/**/*`）

#### ✅ `tsconfig.node.json` (保留)
**用途**：Node.js 环境的 TypeScript 配置（如 `vite.config.ts`）

#### ❌ `tsconfig.tsbuildinfo` (已忽略)
**说明**：TypeScript 增量编译缓存文件
**处理**：已添加到 `.gitignore`

#### ❌ `tsconfig.node.tsbuildinfo` (已忽略)
**说明**：Node 环境的增量编译缓存
**处理**：已添加到 `.gitignore`

---

### Vite 相关

#### ✅ `vite.config.ts` (保留 - 唯一配置)
**用途**：Vite 的主配置文件（TypeScript 版本）
**内容**：
```typescript
export default defineConfig({
  plugins: [
    vue(),
    AutoImport({ ... }),
    Components({ ... })
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  }
})
```

#### ❌ `vite.config.js` (已删除)
**说明**：这是 `vite.config.ts` 编译后的 JavaScript 版本
**问题**：
- 重复文件
- Vite 会直接读取 `.ts` 文件
- 可能是之前运行 `tsc` 或构建命令生成的

**处理**：已删除，并添加到 `.gitignore`

#### ❌ `vite.config.d.ts` (已删除)
**说明**：自动生成的类型定义文件
**问题**：自动生成，不需要手动维护
**处理**：已删除，并添加到 `.gitignore`

---

## 🔍 为什么会有这些文件？

### 1. `vite.config.js` 的来源
可能是以下原因之一：
```bash
# 1. 运行了 TypeScript 编译器
tsc

# 2. 某些构建工具生成
npm run build

# 3. IDE 自动编译
# WebStorm/VSCode 可能在后台编译 TS 文件
```

### 2. `*.tsbuildinfo` 的来源
TypeScript 的 `--incremental` 模式会生成这些文件：
```json
// tsconfig.json
{
  "compilerOptions": {
    "incremental": true  // 会生成 .tsbuildinfo
  }
}
```

---

## ✅ 最终文件结构

### 应该提交到 Git 的文件
```
.nvmrc                 ✅ Node 版本
package.json           ✅ 依赖管理 + 版本要求
tsconfig.json          ✅ TS 主配置
tsconfig.node.json     ✅ TS Node 配置
vite.config.ts         ✅ Vite 配置（唯一）
```

### 应该被忽略的文件（已添加到 .gitignore）
```
*.tsbuildinfo          ❌ TS 增量缓存
vite.config.js         ❌ TS 编译产物
vite.config.d.ts       ❌ 自动生成类型
types/auto-imports.d.ts    ❌ 自动导入类型
types/components.d.ts      ❌ 组件类型
```

---

## 🎯 最佳实践建议

### 1. 清理工作区
```bash
# 删除所有生成的文件
rm -f *.tsbuildinfo vite.config.js vite.config.d.ts

# 重新生成类型定义
npm run type-check
```

### 2. 提交前检查
```bash
# 查看将要提交的文件
git status

# 确保没有 .tsbuildinfo 或 .js/.d.ts 编译产物
git ls-files | grep -E "\.(tsbuildinfo|js|d\.ts)$"
```

### 3. 团队协作
确保 `.gitignore` 包含：
```gitignore
# TypeScript
*.tsbuildinfo

# Vite generated
vite.config.js
vite.config.d.ts

# Auto-generated types
types/auto-imports.d.ts
types/components.d.ts
```

---

## 📚 相关文档

- [TypeScript - Project References](https://www.typescriptlang.org/docs/handbook/project-references.html)
- [Vite - Configuring Vite](https://vitejs.dev/config/)
- [nvm - Node Version Manager](https://github.com/nvm-sh/nvm)
- [package.json engines](https://docs.npmjs.com/cli/v10/configuring-npm/package-json#engines)

