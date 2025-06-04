# Nidalee - League of Legends Game Assistant

<div align="center">
  <img src="assets/logo.png" alt="Nidalee Logo" width="200">
  
  ![License](https://img.shields.io/badge/license-MIT-blue.svg)
  ![Tauri](https://img.shields.io/badge/tauri-2.0.0--alpha-green.svg)
  ![Vue](https://img.shields.io/badge/vue-3.x-brightgreen.svg)
  ![Rust](https://img.shields.io/badge/rust-1.75-orange.svg)
  
  [English](./README_EN.md) | [简体中文](./README.md)
</div>

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

1. Clone the repository
```bash
git clone https://github.com/yourusername/nidalee.git
cd nidalee
```

2. Install dependencies
```bash
# Install frontend dependencies
pnpm install

# Install Rust dependencies
cd src-tauri
cargo build
```

3. Run development environment
```bash
pnpm tauri dev
```

4. Build production version
```bash
pnpm tauri build
```

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

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎯 Roadmap

- [ ] Multi-language Support
- [ ] Match History Export
- [ ] Game Replay Analysis
- [ ] Champion Combo Tips
- [ ] Real-time Voice Alerts
- [ ] Enhanced Data Visualization

## ⚠️ Disclaimer

This project is for learning and research purposes only. Not for commercial use. Users bear all consequences of using this software.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Rust](https://www.rust-lang.org/)
- [League Client Protocol](https://developer.riotgames.com/) 