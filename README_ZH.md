# 🎮 Nidalee

![Nidalee Logo](src/assets/logo.svg)

高性能、体积小巧的智能英雄联盟游戏助手。

Nidalee 是一款专为英雄联盟玩家设计的智能助手，集自动接受匹配、自动选/禁英雄、实时数据分析与个性化设置于一体，助你高效上分，安全合规无外挂风险。基于 Rust + Tauri，启动快、资源占用低，体积小巧。

[![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/license-CC%20BY--NC--SA%204.0-orange.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode.zh-Hans)
![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS-blue.svg)
![Tauri](https://img.shields.io/badge/tauri-2.x-green.svg)
![Vue](https://img.shields.io/badge/vue-3.x-brightgreen.svg)
![Rust](https://img.shields.io/badge/rust-1.75-orange.svg)
[![版本](https://img.shields.io/github/v/release/codeXcn/Nidalee?sort=semver&display_name=tag)](https://github.com/codeXcn/Nidalee/releases/latest)
[![下载量](https://img.shields.io/github/downloads/codeXcn/Nidalee/total)](https://github.com/codeXcn/Nidalee/releases)

简体中文 | [English](./README.md)

---

## ⏱️ 快速上手

1. 在 [Releases 页面](https://github.com/codeXcn/Nidalee/releases/latest) 下载适合你平台的安装包
2. 完成安装并启动应用；启动并登录英雄联盟客户端
3. 应用会自动连接到客户端；侧边栏可查看更新状态

若任意时刻自动更新失败，可在应用内点击「前往手动下载」，或访问 [Releases 页面](https://github.com/codeXcn/Nidalee/releases/latest)。

## ✨ 特性

- 🤖 自动化功能：自动接受匹配、自动选择/禁用英雄
- 📊 数据分析：实时游戏数据分析和统计
- 🎯 个性化设置：可自定义的游戏助手配置、生涯背景配置等
- 🔒 安全可靠：直接与 League Client API 交互，无需第三方工具

## 快速预览

![侧边栏更新流程 - 占位示意](docs/images/update-flow.svg)

更新流程：检查 → 下载 → 安装；在任何失败时提供「前往手动下载」兜底。

![对局分析界面 - 占位示意](docs/images/match-analysis.svg)

对局分析：队伍数据、对线对位、参考建议等洞察。

## 💡 为什么选择 Nidalee

- 面向 LoL 玩家核心流程：接受匹配、选/禁、对局洞察，一站式、轻量、上手快。
- Rust + Tauri 带来极快启动与低资源占用，长期驻留更友好。
- 采用 shadcn-vue + Tailwind v4 的现代化 UI，深/浅色一致且易扩展。
- 明确的分发与签名策略，仅信任官方发布，供应链安全可信。

### 它是什么 / 不是什么

- 它是：基于官方 LCU 的桌面助手；自动化仅限客户端公开接口；专注选/禁与数据洞察。
- 它不是：内存/进程注入、抓包篡改、脚本或任何外挂工具。

## 系统概览

- 轻量与性能：启动快、占用低，适合长期驻留，不打扰。
- 自动更新：启动静默检查，侧边栏一键更新，失败时提供「前往手动下载」兜底。
- 安全分发：仅信任官方发布；安装包附带签名（.sig），更新前校验来源与完整性。
- 现代 UI 与主题：一致的深浅色与 OKLCH 色彩；界面简洁、可读性强。
- 稳定与扩展：组合式与模块化设计，便于后续功能扩展。
- 维护者：签名与安全见 `docs/tauri-signing.md`。

## 📦 下载安装

- 最新版下载：<https://github.com/codeXcn/Nidalee/releases/latest>
- 在「Assets」中按平台下载：
  - Windows：`.msi`
  - macOS：`.dmg`

> 小贴士：若应用内更新失败，可直接打开 [Releases 页面](https://github.com/codeXcn/Nidalee/releases/latest) 手动下载安装。

系统要求：

- Windows 10/11（x64）
- macOS 12+（Intel / Apple Silicon）
- 建议：Windows 上以管理员身份运行（安装/更新更顺畅）

平台一览：

| 平台    | CPU 架构                    | 安装包 |
|---------|-----------------------------|--------|
| Windows | x64                          | .msi   |
| macOS   | Intel、Apple Silicon（M 系） | .dmg   |

### 安装说明（Windows）

1. 下载 `.msi` 安装包
2. 双击运行，按提示完成安装
3. 启动应用，开机即会静默检查更新（侧边栏显示更新提示与进度）

### 安装说明（macOS）

1. 下载 `.dmg` 并打开
2. 将应用拖拽到「应用程序」（Applications）
3. 首次启动若被 Gatekeeper 拦截：在应用图标上右键 → 打开；或前往系统设置 → 隐私与安全性 → 允许打开

## 🚀 开发

### 环境要求

- Node.js 20+
- pnpm 10+
- Rust 1.70+
- Tauri CLI 2.0+

### 本地开发

```bash
git clone https://github.com/codeXcn/Nidalee.git
cd Nidalee

# 安装依赖
pnpm install

# 开发模式（Tauri）
pnpm tauri dev

# 构建生产版本（Tauri）
pnpm tauri build
```

### 项目结构

```text
Nidalee/
├── src/                    # Vue.js 前端代码
├── src-tauri/              # Tauri Rust 后端代码
├── .github/workflows/      # GitHub Actions CI/CD
├── dist/                   # 构建输出
└── docs/                   # 项目文档
```

## 📋 功能清单

- [x] 基础架构搭建
- [x] League Client API 集成
- [x] CI/CD 自动化发布
- [x] 用户信息获取和展示
- [x] 召唤师特征分析
- [x] 自动接受匹配功能
- [x] 自动选择/禁用英雄
- [x] 游戏数据分析
- [x] 个性化设置界面
- [ ] 多语言支持

## 📖 文档导航

完整文档请访问 [docs/](docs/) 或查看 [文档中心](docs/README.md)。

### 快速链接

- **用户文档**
  - [故障排查指南](docs/troubleshooting.md)
  - [用户指南（中文）](docs/user-guide-zh.md)
  - [User Guide (English)](docs/user-guide.md)

- **开发者文档**
  - [贡献指南](docs/contributing.md)
  - [发布指南](docs/release.md)
  - [变更日志](docs/changelog.md)

- **维护者文档**
  - [Tauri 签名与安全](docs/tauri-signing.md)

## 🤝 贡献

欢迎为项目做出贡献！请阅读我们的 [贡献指南](docs/contributing.md) 了解详细信息：

- 开发流程
- 代码规范
- 提交规范（Conventional Commits）
- 分支命名规则
- Pull Request 流程

### 贡献者快速开始

1. Fork 并克隆：`git clone https://github.com/<yourname>/Nidalee.git`
2. 创建功能分支：`git checkout -b feature/your-feature`
3. 安装并运行：`pnpm install && pnpm tauri dev`
4. 进行修改并测试
5. 运行检查：`pnpm lint && pnpm type-check`
6. 提交 PR 并附上清晰的说明

发布流程详见 [发布指南](docs/release.md)。

## 🌐 网络与下载说明

- 默认从 GitHub Releases 下载，可能受网络环境影响较慢或失败。
- 若遇到下载缓慢或失败，应用会提供「前往手动下载」按钮，跳转官方 Releases 最新页进行下载安装。
- 后续将视情况提供额外的国内镜像渠道（如提供，会在此处与应用内提示中同步说明）。

## 🛠️ 故障排查

- 更新失败 / 进度长时间卡住：点击提示中的「前往手动下载」，或访问 [Releases 页面](https://github.com/codeXcn/Nidalee/releases/latest) 获取最新安装包。
- Windows SmartScreen：点击“更多信息”→“仍要运行”，或右键文件→属性→取消阻止后重试。
- macOS Gatekeeper：系统设置 → 隐私与安全性 → 允许打开；或在 Finder 中右键应用 → 打开。
- 无法连接 LCU：确保 LOL 客户端已登录；必要时重启客户端与应用（Windows 建议以管理员身份运行）。
- 权限/写入错误：以管理员身份运行；若仍失败，手动下载安装；确保安装目录可写。

## 📄 许可证

本项目采用 [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode.zh-Hans)（署名-非商业性使用-相同方式共享）国际许可协议。

- 允许自由复制、分发、演绎，但禁止任何商业用途。
- 衍生作品必须采用相同协议。
- 使用时请注明原作者及项目地址。

完整条款详见 LICENSE 文件。

## ⚠️ 免责声明

本项目仅作为英雄联盟玩家的辅助工具，所有功能均基于 Riot Games 官方公开的 League Client API（LCU API）和客户端本地数据实现。

本工具不进行任何游戏内存、进程、网络数据的篡改，不注入、不修改、不破解游戏客户端，也不具备任何外挂、作弊、加速、脚本等功能。

- 本项目严格遵守英雄联盟用户协议及相关法律法规，仅供学习、研究和个人娱乐用途。
- 所有数据交互均通过官方 API 完成，未对游戏客户端、服务器或数据包进行任何非官方操作。
- 本工具不会收集、上传或泄露用户的任何隐私信息、账号密码等敏感数据。
- 本项目与 Riot Games 及腾讯公司无任何直接或间接关联，亦未获得其官方授权。
- 使用本工具所产生的任何后果（包括但不限于账号风险、数据丢失、功能异常等），开发者不承担任何法律责任和经济责任。
- 本项目及其所有衍生作品严禁任何商业用途，所有贡献和再分发须采用相同协议。

---

Built with ❤️ using [Tauri 2.0](https://tauri.app/) + [Vue.js](https://vuejs.org/)
