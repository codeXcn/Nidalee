# 🎮 Nidalee - 英雄联盟游戏助手

> 功能强大的英雄联盟游戏助手，提供实时游戏数据分析、自动化功能和个性化设置。

![Build Status](https://github.com/your-username/Nidalee/workflows/Release/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)

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
git clone https://github.com/your-username/Nidalee.git
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
- [ ] 自动接受匹配功能
- [ ] 自动选择/禁用英雄
- [ ] 游戏数据分析
- [ ] 个性化设置界面
- [ ] 多语言支持

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

**Built with ❤️ using [Tauri](https://tauri.app/) + [Vue.js](https://vuejs.org/)**
