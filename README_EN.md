<div align="center">
  <img src="src/assets/logo.svg" alt="Nidalee Logo" width="120" height="120">

  <h1>🎮 Nidalee</h1>
  <p><strong>High-performance, lightweight and intelligent League of Legends assistant</strong></p>
  <p>Nidalee is a high-performance, lightweight and intelligent assistant for League of Legends players. It integrates auto-accept, auto pick/ban, real-time data analysis and personalized settings, helping you climb the ranks efficiently and safely. Powered by Rust + Tauri, it features fast startup, low resource usage, and a minimal footprint.</p>

  <div>
    <a href="https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode" target="_blank"><img src="https://img.shields.io/badge/license-CC%20BY--NC--SA%204.0-orange.svg" alt="License"/></a>
    <img src="https://img.shields.io/badge/tauri-2.0.0--alpha-green.svg" alt="Tauri">
    <img src="https://img.shields.io/badge/vue-3.x-brightgreen.svg" alt="Vue">
    <img src="https://img.shields.io/badge/rust-1.75-orange.svg" alt="Rust">
    <img src="https://img.shields.io/badge/platform-Windows-blue.svg" alt="Platform">
  </div>

  <br>

  <p>
    <a href="#-features">✨ Features</a> •
    <a href="#-installation">📦 Installation</a> •
    <a href="#-development">🚀 Development</a> •
    <a href="#-usage">📖 Usage</a> •
    <a href="#-contributing">🤝 Contributing</a>
  </p>

  <p>
    <a href="./README.md">简体中文</a> | <a href="./README_EN.md">English</a>
  </p>
</div>

---

## 🌟 Features

### 🎮 Automation

- **Auto Accept**: Automatically detect and accept match invitations
- **Smart Champion Selection**: Auto pick or ban champions based on presets
- **Rune Configuration**: Automatically fetch and apply optimal rune pages from OP.GG

### 📊 Data Analysis

- **Real-time Match Analysis**
  - Team Composition Evaluation
  - Lane Advantage Analysis
  - Teamfight Capability Score
  - Intelligent Tactical Suggestions
- **Player Statistics**
  - KDA and Win Rate Analysis
  - Position Preference Analysis
  - Champion Pool Analysis
  - Recent Performance Score

### 🔍 Information Display

- **Real-time Match Information**
  - Teammate and Opponent Details
  - Champion Counter Relationships
  - Player Match History
- **Data Visualization**
  - Lane Advantage Indicator
  - Team Strength Comparison
  - Player Stats Radar Chart

## 🚀 Tech Stack

- **Frontend**: Vue 3 + TypeScript
- **Backend**: Rust + Tauri
- **Communication**: LCP (League Client Protocol)
- **State Management**: Vue Composition API
- **UI Framework**: Custom Components

## 📦 Installation

Download the latest Windows version from the [Releases](../../releases) page:

| Platform | File | Description |
|----------|------|-------------|
| **Windows** | `Nidalee_1.0.0_x64_en-US.msi` | Windows 64-bit installer |

### Installation Steps (Windows)

1. Download the `.msi` file
2. Double-click to run the installer
3. Follow the wizard to complete installation

## 🔧 Configuration

1. Game Client Configuration
   - Auto-detect LCU authentication info
   - Support custom port and token

2. Feature Module Configuration
   - Auto accept match toggle
   - Champion selection presets
   - Rune page auto-update settings

## 📝 Usage Guide

1. **Auto Accept Match**
   - Automatically monitor and accept match invitations when enabled
   - Configurable accept delay

2. **Champion Selection**
   - Preset priority champions
   - Set auto-ban champions
   - Team composition-based recommendations

3. **Rune Configuration**
   - Auto-fetch and apply recommended runes
   - Support custom rune schemes
   - Quick switch between champion runes

4. **Data Analysis**
   - Real-time match analysis
   - Team advantage/disadvantage overview
   - Tactical suggestions

## 🤝 Contributing

Contributions are welcome! Check out our [Contributors List](CONTRIBUTORS.md).

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode) (Attribution-NonCommercial-ShareAlike 4.0 International) license.

- Free to copy, distribute, and adapt, but **commercial use is strictly prohibited**.
- Derivative works must use the same license.
- Please credit the original author and project link when using.

See the LICENSE file for full terms.

## ⚠️ Disclaimer

This project is for League of Legends players as an auxiliary tool. All features are based on Riot Games' official League Client API (LCU API) and local client data.

**This tool does not modify, inject, or tamper with game memory, processes, or network data, nor does it provide any cheating, acceleration, or scripting functions.**

- Strictly for learning, research, and personal entertainment only.
- All data interactions are via official APIs; no unofficial operations on the game client, server, or packets.
- No collection, upload, or leakage of any user privacy or sensitive data.
- This is open source software, not affiliated with Riot Games or Tencent, nor officially authorized.
- The developer assumes no legal or financial responsibility for any consequences (including but not limited to account risk, data loss, or functional issues) arising from use.
- **Commercial use of this project and all derivatives is strictly prohibited. All contributions and redistribution must use the same license.**

**Please ensure your use complies with the League of Legends user agreement and related policies. If in doubt, stop using and consult official support.**

## 🙏 Acknowledgments

- [Tauri 2.0](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Rust](https://www.rust-lang.org/)
- [League Client Protocol](https://developer.riotgames.com/)
