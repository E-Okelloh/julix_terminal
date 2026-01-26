# 🚀 RusTerm - A Modern Terminal Emulator Built with Rust

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=for-the-badge)

**A blazingly fast, memory-safe terminal emulator written in Rust**

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Architecture](#-architecture) • [Contributing](#-contributing)

</div>

---

## 📖 About

RusTerm is a modern, cross-platform terminal emulator built from the ground up in Rust. Designed with performance, safety, and extensibility in mind, RusTerm aims to provide a superior terminal experience with GPU-accelerated rendering, full VT100/xterm compatibility, and a rich feature set.

### Why RusTerm?

- ⚡ **Blazingly Fast**: GPU-accelerated rendering for smooth 60+ FPS performance
- 🔒 **Memory Safe**: Written in Rust - no buffer overflows or memory leaks
- 🎨 **Beautiful**: TrueColor support, ligatures, and customizable themes
- 🌍 **Cross-Platform**: Works seamlessly on Linux, macOS, and Windows
- 🔧 **Extensible**: Plugin system and extensive configuration options
- 📦 **Lightweight**: Minimal resource footprint despite rich features

---

## ✨ Features

### Core Functionality
- ✅ Full VT100/VT220/xterm escape sequence support
- ✅ 256-color and 24-bit TrueColor support
- ✅ Unicode and UTF-8 text rendering
- ✅ Mouse support (click, drag, scroll)
- ✅ Text selection and clipboard integration
- ✅ Scrollback buffer with unlimited history
- ✅ Dynamic window resizing

### Advanced Features
- 🎯 **GPU-Accelerated Rendering**: Powered by wgpu for buttery-smooth performance
- 🔤 **Font Ligatures**: Programming ligatures for better code readability
- 🖼️ **Image Protocol**: Display images inline (Sixel, iTerm2, Kitty protocols)
- 📑 **Tabs & Splits**: Multiple terminals in one window
- ⌨️ **Configurable Keybindings**: Customize every keyboard shortcut
- 🎨 **Theme System**: 100+ built-in themes, custom theme support
- 🔔 **Visual Bell**: Non-intrusive notifications
- 📊 **Performance Metrics**: Built-in FPS counter and diagnostics

### Developer-Friendly
- 🔍 **Debug Mode**: Inspect escape sequences and rendering pipeline
- 📝 **Session Logging**: Record and replay terminal sessions
- 🔌 **Plugin API**: Extend functionality with Rust plugins
- 📚 **Comprehensive Docs**: Detailed documentation and examples

---

## 🚀 Installation

### From Binary (Recommended)

Download the latest release for your platform:
```bash
