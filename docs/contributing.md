# 🤝 贡献指南

欢迎为 Nidalee 项目做出贡献！本文档将指导你如何参与项目开发。

## 📚 目录

- 如何贡献
- 开发流程
- 代码规范
- 提交规范
- 分支命名
- 贡献类型
- 项目维护者
- 获取帮助
- 许可证

## 🚀 如何贡献

### 1. Fork 并克隆项目

```bash
# Fork 本仓库后克隆到本地
git clone https://github.com/<yourname>/Nidalee.git
cd Nidalee
```

### 2. 安装依赖并运行

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 代码检查
pnpm lint
pnpm type-check

# 构建（可选）
pnpm tauri build
```

### 3. 创建分支

根据你要做的工作类型创建对应的分支（见下方“分支命名”章节）。

```bash
git checkout -b feature/your-feature-name
```

### 4. 开发与测试

- 编写代码并确保功能正常
- 添加必要的测试用例
- 运行 `pnpm lint` 和 `pnpm type-check` 确保代码质量
- 涉及 UI 的改动请提供截图或 GIF

### 5. 提交代码

遵循下方“提交规范”（Conventional Commits）提交代码。

```bash
git add .
git commit -m "feat(scope): add new feature"
git push origin feature/your-feature-name
```

### 6. 创建 Pull Request

- 在 GitHub 上创建 PR
- 提供清晰的变更说明
- 附上相关的 Issue 编号
- UI 改动需附上截图或 GIF

## 🔧 开发流程

### 环境要求

- Node.js 20+
- pnpm 10+
- Rust 1.70+
- Tauri CLI 2.0+

### 项目结构

```text
Nidalee/
├── src/                    # Vue.js 前端代码
│   ├── components/         # 组件
│   ├── composables/        # 组合式函数
│   ├── stores/             # 状态管理
│   ├── views/              # 页面视图
│   └── ...
├── src-tauri/              # Tauri Rust 后端
│   ├── src/                # Rust 源码
│   └── tauri.conf.json     # Tauri 配置
├── docs/                   # 项目文档
└── .github/workflows/      # CI/CD 配置
```

## 📐 代码规范

### TypeScript/Vue

- 使用 ESLint 和 Prettier 进行代码格式化
- 遵循 Vue 3 Composition API 最佳实践
- 使用 TypeScript 类型注解
- 组件使用 PascalCase 命名
- 文件使用 kebab-case 命名

### Rust

- 遵循 Rust 官方代码风格指南
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 进行代码检查

## 📝 提交规范

本项目遵循 Conventional Commits 规范。

### 基本格式

```text
type(scope): subject

[optional body]

[optional footer]
```

### Type 类型

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档变更
- `style`: 代码格式（不影响代码运行）
- `refactor`: 重构（既不是新功能也不是修复）
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具/依赖等杂项
- `ci`: CI/CD 配置变更
- `build`: 构建系统或外部依赖变更

### Scope 范围（建议）

- `updater`: 更新模块
- `lcu`: League Client API 相关
- `store`: 状态管理
- `ui`: 用户界面
- `tray`: 系统托盘
- `match`: 对局相关
- `settings`: 设置相关

### 示例（提交信息）

```text
feat(updater): add sidebar manual update entry with progress
fix(lcu): retry on 401 and refresh auth token
docs(readme): update installation instructions
refactor(store): split ui store into modules
perf(match): optimize player data loading
test(utils): add date formatting tests
chore(deps): update dependencies
```

### 注意事项

- subject 不超过 50 字符
- 使用祈使句（"add" 而不是 "added" 或 "adds"）
- 首字母小写
- 结尾不加句号

## 🌿 分支命名

使用以下格式命名分支：`<type>/<scope>-<short-desc>`

### 分支类型

- `feature/`: 新功能
- `fix/`: Bug 修复
- `docs/`: 文档更新
- `refactor/`: 代码重构
- `perf/`: 性能优化
- `test/`: 测试相关
- `chore/`: 杂项

### 示例（分支）

```text
feature/updater-ui
fix/lcu-auth-retry
docs/update-readme
refactor/store-modules
perf/table-render
test/utils-date
chore/ci-cache
```

### 命名规则

- 使用小写字母
- 使用短横线 `-` 分隔单词
- scope 对应子模块或目录名
- 描述简洁明了

## 📋 贡献类型

我们欢迎以下类型的贡献：

- 🐛 Bug 修复
- ✨ 新功能开发
- 📝 文档改进
- 🎨 UI/UX 优化
- 🔧 工具和脚本
- ⚡ 性能优化
- 🧪 测试用例
- 🌐 国际化
- ♿ 无障碍

## 👑 项目维护者

### 核心团队

- **CodexLin** - 项目创始人与主要维护者。GitHub: [@codexlin](https://github.com/codexlin)。负责：核心架构设计、后端开发、数据分析实现。

### 如何成为维护者

长期贡献者可能被邀请成为项目维护者。维护者需要：

- 对项目有深入理解
- 持续稳定的贡献记录
- 良好的代码审查能力
- 积极参与社区讨论

## 💡 特别感谢

感谢以下项目和社区的支持：

- [Tauri](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Rust](https://www.rust-lang.org/)
- [shadcn-vue](https://www.shadcn-vue.com/)
- [Tailwind CSS](https://tailwindcss.com/)

## 📞 获取帮助

如果你在贡献过程中遇到问题：

- 查看 [文档首页](./README.md)
- 在 [Issues](https://github.com/codeXcn/Nidalee/issues) 中搜索类似问题
- 创建新的 Issue 提问
- 查看 [Discussions](https://github.com/codeXcn/Nidalee/discussions) 参与讨论

## 📄 许可证

通过贡献代码，你同意你的贡献将在 [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode.zh-Hans) 许可证下发布。

---

我们会定期更新此文档。如果你对贡献流程有建议，欢迎提交 PR 改进本文档。
