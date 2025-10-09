<!-- markdownlint-disable MD033 MD041 -->
<div align="center">
  <img src="src/assets/logo.svg" alt="Nidalee Logo" width="120" height="120">

  <h1>🎮 Nidalee</h1>
  <p><strong>High-performance, lightweight and intelligent League of Legends assistant</strong></p>
  <p>Nidalee is a high-performance, lightweight and intelligent assistant for League of Legends players. It integrates auto-accept, auto pick/ban, real-time data analysis and personalized settings, helping you climb the ranks efficiently and safely. Powered by Rust + Tauri, it features fast startup, low resource usage, and a minimal footprint.</p>

  <div>
    <a href="https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode" target="_blank"><img src="https://img.shields.io/badge/license-CC%20BY--NC--SA%204.0-orange.svg" alt="License"/></a>
    <img src="https://img.shields.io/badge/tauri-2.x-green.svg" alt="Tauri">
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
    <a href="#-updates">🔄 Updates</a> •
    <a href="#-distribution--signature-policy">📦 Distribution & Signature Policy</a> •
    <a href="#-network--download">🌐 Network & Download</a> •
    <a href="#-troubleshooting">🛠️ Troubleshooting</a> •
    <a href="#-contributing">🤝 Contributing</a>
  </p>

  <p>

  [简体中文](./README_ZH.md) | English

  </p>
</div>


## 🌟 Features

### 🎮 Automation


### 📊 Data Analysis

  - Team Composition Evaluation
  - Lane Advantage Analysis
  - Teamfight Capability Score
  - Intelligent Tactical Suggestions
  - KDA and Win Rate Analysis
  - Position Preference Analysis
  - Champion Pool Analysis
  - Recent Performance Score

### 🔍 Information Display

  - Teammate and Opponent Details
  - Champion Counter Relationships
  - Player Match History
  - Lane Advantage Indicator
  - Team Strength Comparison
  - Player Stats Radar Chart

## 🚀 Tech Stack


## 📦 Installation

Download the latest Windows version from the [Releases](../../releases) page:

| Platform | File | Description |
|----------|------|-------------|
| **Windows** | `Nidalee_1.0.0_x64_en-US.msi` | Windows 64-bit installer |

### Installation Steps (Windows)

1. Download the `.msi` file
2. Double-click to run the installer
3. Launch the app; it will automatically check for updates (the sidebar shows the update prompt and progress)

> Important: It's recommended to run the app as Administrator (right-click → Run as administrator) to ensure sufficient permissions for auto updates, log writing, and network port binding.

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


See the LICENSE file for full terms.

## ⚠️ Disclaimer

This project is for League of Legends players as an auxiliary tool. All features are based on Riot Games' official League Client API (LCU API) and local client data.

**This tool does not modify, inject, or tamper with game memory, processes, or network data, nor does it provide any cheating, acceleration, or scripting functions.**


**Please ensure your use complies with the League of Legends user agreement and related policies. If in doubt, stop using and consult official support.**

## 🙏 Acknowledgments


## 🔄 Updates

This project integrates Tauri v2 Updater to automatically check, download, install and relaunch.

- Backend: `tauri-plugin-updater` and `tauri-plugin-process` are registered.
- Frontend: `@tauri-apps/plugin-updater` and `@tauri-apps/plugin-process` are installed, and the app performs a silent update check on startup.
- Capabilities: `updater:default` and `process:default` are granted.

Runtime behavior:

- On startup, the app silently checks for updates. When a new version is available, the sidebar shows “Update available”. Clicking it starts download and installation, followed by auto relaunch.
- During download, the sidebar shows a progress bar and dynamic status text:
  - 0%: Connecting to update server…
  - 1–99%: Downloading N%
  - 100%: Installing / preparing to relaunch…
- If update fails, a toast appears with a “Go to manual download” action to open the latest Releases page.

## 📦 Distribution & Signature Policy

- Official builds and update artifacts are published only in this repository’s [Releases](https://github.com/codeXcn/Nidalee/releases).
- The app’s updater only trusts the official signing public key configured in `src-tauri/tauri.conf.json`. Third-party builds cannot use in-app auto updates.
- Publishing builds or redistributing packaged artifacts outside of this repository is not allowed. Please participate via Issues/PRs and release here.

## 🌐 Network & Download

- Downloads use GitHub Releases and may be slow or unstable depending on network conditions.
- If downloads are slow or fail, use the “Go to manual download” action to open the official Releases page and install manually.
- Additional region-friendly mirrors may be provided in the future; they will be announced here and in the app if available.

## 🛠️ Troubleshooting

- Update failure or stuck progress: Click “Go to manual download” in the toast and install from the official Releases page.
- Windows SmartScreen warning: Click “More info” → “Run anyway” or unblock the file from Properties.
- Cannot connect to LCU: Ensure the League client is logged in; restart the client if needed(Run as administrator).
- Insufficient permissions / update failure / unable to write: Run the app as Administrator (right-click → Run as administrator). If update still fails, retry in admin mode or download from Releases and install manually. Ensure the installation directory is writable.

```bash
pnpm install

# Development (Tauri)
pnpm exec tauri dev

# Production build (Tauri)
pnpm exec tauri build
```
