# Nidalee - 英雄联盟游戏助手

<div align="center">
  <img src="assets/logo.png" alt="Nidalee Logo" width="200">
  
  ![License](https://img.shields.io/badge/license-MIT-blue.svg)
  ![Tauri](https://img.shields.io/badge/tauri-2.0.0--alpha-green.svg)
  ![Vue](https://img.shields.io/badge/vue-3.x-brightgreen.svg)
  ![Rust](https://img.shields.io/badge/rust-1.75-orange.svg)
  
  [English](./README_EN.md) | [简体中文](./README.md)
</div>

## 🌟 功能特点

### 🎮 自动化操作
- **自动接受对局**: 自动检测并接受对局邀请
- **智能英雄选择**: 根据预设自动选择或禁用英雄
- **符文页配置**: 自动从 OP.GG 获取并应用最优符文配置

### 📊 数据分析
- **实时对局分析**
  - 队伍阵容评估
  - 对线优势分析
  - 团战能力评分
  - 智能战术建议
- **玩家数据统计**
  - KDA 和胜率分析
  - 位置偏好分析
  - 英雄池分析
  - 近期表现评分

### 🔍 信息展示
- **实时对局信息**
  - 队友和对手详细信息
  - 英雄克制关系
  - 玩家历史战绩
- **可视化数据展示**
  - 对线优势指示器
  - 团队实力对比图
  - 选手数据雷达图

## 🚀 技术栈

- **前端**: Vue 3 + TypeScript
- **后端**: Rust + Tauri
- **通信**: LCP (League Client Protocol)
- **状态管理**: Vue Composition API
- **UI 框架**: 自定义组件

## 📦 安装说明

1. 克隆仓库
```bash
git clone https://github.com/yourusername/nidalee.git
cd nidalee
```

2. 安装依赖
```bash
# 安装前端依赖
pnpm install

# 安装 Rust 依赖
cd src-tauri
cargo build
```

3. 运行开发环境
```bash
pnpm tauri dev
```

4. 构建生产版本
```bash
pnpm tauri build
```

## 🔧 配置说明

1. 游戏客户端配置
   - 自动获取 LCU 认证信息
   - 支持自定义端口和令牌

2. 功能模块配置
   - 自动接受对局开关
   - 英雄选择预设
   - 符文页自动更新设置

## 📝 使用说明

1. **自动接受对局**
   - 启用功能后自动监测并接受对局邀请
   - 可设置延迟接受时间

2. **英雄选择**
   - 预设优先选择英雄
   - 设置自动禁用英雄
   - 根据队伍阵容推荐英雄

3. **符文配置**
   - 自动获取并应用推荐符文
   - 支持自定义符文方案
   - 快速切换不同英雄符文

4. **数据分析**
   - 实时查看对局分析
   - 了解队伍优劣势
   - 获取战术建议

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！查看 [贡献者列表](CONTRIBUTORS.md) 了解项目贡献者。

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🎯 开发路线图

- [ ] 多语言支持
- [ ] 战绩数据导出
- [ ] 对局录像分析
- [ ] 英雄连招提示
- [ ] 实时语音提醒
- [ ] 数据可视化增强

## ⚠️ 免责声明

本项目仅用于学习和研究目的，请勿用于商业用途。使用本软件产生的任何后果由用户自行承担。

## 🙏 鸣谢

- [Tauri](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Rust](https://www.rust-lang.org/)
- [League Client Protocol](https://developer.riotgames.com/)
