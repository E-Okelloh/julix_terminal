# Julix

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Status](https://img.shields.io/badge/status-active-development-orange.svg?style=for-the-badge)

A lightweight, modern terminal emulator built with Rust.

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Architecture](#architecture) • [Contributing](#contributing)

</div>

---

## About

Julix is a cross-platform terminal emulator written in Rust. It starts a real shell in a PTY, parses ANSI escape sequences, and renders the terminal in a terminal UI using `crossterm`.

The project is intentionally focused on a clean and maintainable core: a PTY-backed shell session, a VT parser, and a simple rendering loop that can be extended over time.

### Why Julix?

- Rust-first design with strong safety guarantees
- Built around real PTY integration for shell compatibility
- Lightweight architecture with a small dependency footprint
- Designed for extensibility as the project grows
- Easy to run locally with a standard Rust toolchain

---

## Features

### Core functionality

- PTY-backed shell session
- ANSI/VT escape sequence handling via `vte`
- Cursor movement, line wrapping, and screen clearing
- Resizable terminal window
- Keyboard input forwarding for common terminal keys
- Basic rendering pipeline for terminal output

### Current capabilities

- Shell startup using the user's default shell (`$SHELL` on Unix, `cmd.exe` on Windows)
- Input handling for characters, Enter, Backspace, Tab, navigation keys, and Ctrl shortcuts
- Automatic redraw loop with terminal refresh timing
- Scrollback-style screen management for terminal output

### Planned improvements

- Improved color and text styling support
- More complete terminal emulator feature parity
- Scrollback buffer and history navigation
- Tabs and split panes
- Better mouse support and selection
- Theme system and configuration file support

---

## Installation

### Prerequisites

- Rust 1.70+ (or a newer stable version)
- A Unix-like environment or Windows terminal session

### Clone and run

```bash
git clone https://github.com/E-Okelloh/julix_terminal.git
cd julix_terminal
cargo run
```

### Build for release

```bash
cargo build --release
```

---

## Usage

Run the app from the project root:

```bash
cargo run
```

Once launched, Julix will open a shell inside the terminal UI.

Controls:

- `Ctrl+Q` — quit the emulator
- Arrow keys — send standard terminal navigation sequences
- `Enter`, `Tab`, `Backspace` — supported input keys
- Resize the terminal window to update the PTY size

---

## Architecture

The project is organized around a few focused pieces:

- `src/main.rs` — main event loop, rendering, input handling, and exit logic
- `src/pty.rs` — PTY creation, shell startup, resizing, and I/O
- `src/terminal.rs` — terminal state, cursor logic, screen buffer, and VT parser integration

At a high level, the application works like this:

1. Start a shell inside a PTY.
2. Read output from the PTY in a background thread.
3. Feed bytes into the `vte` parser.
4. Update terminal state and render the screen.
5. Forward keyboard input back into the PTY.

---

## Contributing

Contributions are welcome.

If you want to help:

1. Fork the repository.
2. Create a feature branch.
3. Make your changes.
4. Run the project locally with `cargo run`.
5. Open a pull request with a clear explanation of your changes.

### Development notes

```bash
cargo fmt
cargo check
cargo run
```

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
