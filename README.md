
<div align="center">
  <img src="src/assets/logo.svg" alt="Nidalee Logo" width="120" height="120">

  <h1>🎮 Nidalee</h1>
  <p><strong>功能强大的英雄联盟游戏助手</strong></p>
  <p>提供实时游戏数据分析、自动化功能和个性化设置</p>

  <div>
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
    <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platform">
    <img src="https://img.shields.io/badge/tauri-2.0.0--alpha-green.svg" alt="Tauri">
    <img src="https://img.shields.io/badge/vue-3.x-brightgreen.svg" alt="Vue">
    <img src="https://img.shields.io/badge/rust-1.75-orange.svg" alt="Rust">
  </div>

  <br>

  <p>
    <a href="#-特性">✨ 特性</a> •
    <a href="#-下载安装">📦 下载</a> •
    <a href="#-开发">🚀 开发</a> •
    <a href="docs/user-guide-zh.md">📖 使用指南</a> •
    <a href="#-贡献">🤝 贡献</a>
  </p>
</div>

---

## ✨ 特性

- 🤖 **自动化功能**：自动接受匹配、自动选择/禁用英雄
- 📊 **数据分析**：实时游戏数据分析和统计
- 🎯 **个性化设置**：可自定义的游戏助手配置
- 🔒 **安全可靠**：直接与League Client API交互，无需第三方工具
- 🌐 **跨平台支持**：支持 Windows、macOS、Linux

## 📦 下载安装

### 自动发布

前往 [Releases](../../releases) 页面下载最新版本：

| 平台 | 下载文件 | 说明 |
|------|----------|------|
| **Windows** | `Nidalee_1.0.0_x64_en-US.msi` | Windows 64位安装程序 |
| **macOS** | `Nidalee_1.0.0_universal.dmg` | macOS 通用版本（Intel + Apple Silicon） |
| **Linux** | `nidalee_1.0.0_amd64.deb` | Debian/Ubuntu 软件包 |

### 安装说明

#### Windows
1. 下载 `.msi` 文件
2. 双击运行安装程序
3. 按照向导完成安装

#### macOS
1. 下载 `.dmg` 文件
2. 双击打开磁盘映像
3. 拖拽应用到 Applications 文件夹

#### Linux (Debian/Ubuntu)
```bash
# 下载deb包后安装
sudo dpkg -i nidalee_1.0.0_amd64.deb

# 或者使用apt安装依赖
sudo apt install -f
```

## 🚀 开发

### 环境要求

- **Node.js** 18+
- **pnpm** 8+
- **Rust** 1.70+
- **Tauri CLI** 2.0+

### 本地开发

```bash
# 克隆项目
git clone https://github.com/codexlin/Nidalee.git
cd Nidalee

# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

### 项目结构

```
Nidalee/
├── src/                    # Vue.js 前端代码
├── src-tauri/             # Tauri Rust 后端代码
├── .github/workflows/     # GitHub Actions CI/CD
├── dist/                  # 构建输出
└── docs/                  # 项目文档
```

## 📋 功能清单

- [x] 基础架构搭建
- [x] League Client API 集成
- [x] CI/CD 自动化发布
- [x] 用户信息获取和展示
- [x] 召唤师特征分析
- [x] 自动接受匹配功能
- [ ] 自动选择/禁用英雄
- [x] 游戏数据分析
- [x] 个性化设置界面
- [ ] 多语言支持

## 📖 使用指南

详细的使用说明请查看 [使用指南](docs/user-guide-zh.md)

## 🤝 贡献

欢迎提交 Pull Request 或 Issue！

### 发布流程

详见 [发布指南](RELEASE.md)

简要步骤：
1. 更新版本号
2. 创建 git tag: `git tag v1.0.1`
3. 推送 tag: `git push origin v1.0.1`
4. GitHub Actions 自动构建并发布

## 📄 许可证

[MIT License](LICENSE)

## ⚠️ 免责声明

本工具仅供学习和研究使用。使用前请确保遵守《英雄联盟》用户协议和相关法规。

---

**Built with ❤️ using [Tauri 2.0](https://tauri.app/) + [Vue.js](https://vuejs.org/)**
