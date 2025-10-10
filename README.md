# 🎮 Nidalee

![Nidalee Logo](src/assets/logo.svg)

High-performance, lightweight assistant for League of Legends.

Nidalee integrates auto-accept, auto pick/ban, real‑time data insights, and customizable settings to help you climb efficiently and safely. Powered by Rust + Tauri for fast startup and low resource usage.

[![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/license-CC%20BY--NC--SA%204.0-orange.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode)
![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)
![Tauri](https://img.shields.io/badge/tauri-2.x-green.svg)
![Vue](https://img.shields.io/badge/vue-3.x-brightgreen.svg)
![Rust](https://img.shields.io/badge/rust-1.75-orange.svg)

English | [简体中文](./README_ZH.md)

---

## ✨ Features

- 🤖 Automation: auto-accept, auto pick/ban
- � Insights: real-time analysis and statistics
- 🎯 Personalization: flexible presets, profile backgrounds, etc.
- 🔒 Safety: interacts only with official League Client API (LCU)

## ⚡ Quick Preview

![Update flow - placeholder](docs/images/update-flow.svg)

![Match analysis - placeholder](docs/images/match-analysis.svg)

## 💡 Why Nidalee

- Built around LoL’s core flows: accept, pick/ban, match insights; lightweight and easy to use.
- Rust + Tauri deliver fast startup and low footprint for long-running sessions.
- Modern UI via shadcn-vue + Tailwind v4; consistent dark/light themes.
- Clear distribution & signature policy; only official releases are trusted.

## System Overview

- Lightweight & performant: fast startup, low resource usage.
- Auto update: silent check on launch; one-click update in sidebar; fallback to manual download on failure.
- Trusted distribution: official Releases only, with detached signatures (.sig) to verify integrity.
- Modern UI & themes: OKLCH palette, clean and readable design.
- Stable & extensible: modular, composition-friendly codebase.
- Maintainers: see signing & security notes in `docs/tauri-signing.md`.

## 📦 Installation

Download the latest Windows version from [Releases](https://github.com/codeXcn/Nidalee/releases):

| Platform | File | Notes |
|---|---|---|
| Windows | `Nidalee_1.0.0_x64_en-US.msi` | 64‑bit installer |

### Installation (Windows)

1. Download the `.msi`
2. Double‑click to install
3. Launch the app; it will auto‑check for updates (the sidebar shows prompts and progress)

> Important: It’s recommended to run as Administrator (right‑click → Run as administrator) to ensure permissions for updates, logging, and network ports.
> Supported systems: Windows 10/11 (x64). Other platforms may be added later.

## 🚀 Development

### Requirements

- Node.js 20+
- pnpm 10+
- Rust 1.70+
- Tauri CLI 2.0+

### Local Development

```bash
git clone https://github.com/codeXcn/Nidalee.git
cd Nidalee

# Install deps
pnpm install

# Dev (Tauri)
pnpm tauri dev

# Build (Tauri)
pnpm tauri build
```

### Project Structure

```text
Nidalee/
├── src/                    # Vue frontend
├── src-tauri/              # Tauri Rust backend
├── .github/workflows/      # GitHub Actions CI/CD
├── dist/                   # Build output
└── docs/                   # Documentation
```

## 📋 Feature Checklist

- [x] Base scaffolding
- [x] League Client API integration
- [x] CI/CD automated releases
- [x] User info and profiling
- [x] Auto accept match
- [x] Auto pick/ban
- [x] Game data analysis
- [x] Settings UI
- [ ] i18n

## � Usage

See docs in the `docs/` folder. Chinese user guide: [docs/user-guide-zh.md](docs/user-guide-zh.md)

## 🤝 Contributing

PRs and Issues are welcome!

### How to Contribute

1. Fork and clone: `git clone https://github.com/<yourname>/Nidalee.git`
2. Create a feature branch (see naming below)
3. Install & run: `pnpm install && pnpm tauri dev`
4. Before commit, run:
   - Lint & types: `pnpm lint && pnpm type-check`
   - Optional build: `pnpm tauri build`
5. Open a PR with clear description, screenshots/GIFs for UI, and related issues

### Branch Naming

- `feature/<scope>-<desc>`  e.g. `feature/updater-ui`
- `fix/<scope>-<desc>`      e.g. `fix/lcu-auth-retry`
- `docs/<desc>`             e.g. `docs/update-readme`
- `refactor/<scope>-<desc>` e.g. `refactor/store-modules`
- `perf/<scope>-<desc>`     e.g. `perf/table-render`
- `test/<scope>-<desc>`     e.g. `test/utils-date`
- `chore/<desc>`            e.g. `chore/ci-cache`

Conventional Commits are encouraged: `type(scope): subject` (≤ 50 chars).

### Release

See [RELEASE.md](RELEASE.md). Tag‑driven CI builds installers and `latest.json`.

## 🌐 Network & Download

- Downloads use GitHub Releases and may be slow in some regions.
- If a download fails, the app shows “Go to manual download” to open the official Releases page.
- Additional mirrors may be provided later and will be announced here and in‑app.

## 🛠️ Troubleshooting

- Update fails / stuck: use “Go to manual download” and install from Releases.
- Windows SmartScreen: click “More info” → “Run anyway”, or unblock in file Properties.
- Cannot connect to LCU: ensure the League client is running; restart if needed; try running as Administrator; check logs.
- Permissions / write errors: run as Administrator; if still failing, re‑install from Releases; ensure the install path is writable.

## 📄 License

Licensed under [CC BY‑NC‑SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode).

See the LICENSE file for full terms.

## ⚠️ Disclaimer

This project is an auxiliary tool for League of Legends. All features rely on Riot Games’ official League Client API (LCU API) and local client data.

This tool does not modify, inject, or tamper with game memory, processes, or network data, nor provide any cheating/automation beyond LCU‑based interactions.

- Use must comply with the game’s user agreement and relevant policies.
- No personal sensitive data is collected or uploaded by this project.
- This project is not affiliated with Riot Games or Tencent and is not officially endorsed.
- You are solely responsible for any consequences arising from usage.

---

Built with ❤️ using [Tauri 2.0](https://tauri.app/) + [Vue.js](https://vuejs.org/)
