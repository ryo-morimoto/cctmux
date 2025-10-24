# cctmux

**tmux + Claude Code integration manager with TUI dashboard**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/ryo-morimoto/cctmux)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.88+-orange.svg)](https://www.rust-lang.org)

cctmux manages multiple parallel Claude Code sessions in tmux.
It provides real-time status visualization, keyboard-driven navigation, and automatic notifications.

## Features

- Real-time TUI dashboard to view all Claude Code sessions at a glance
- Smart navigation to jump to approval-waiting or error panes with keyboard shortcuts
- Background monitoring daemon that automatically tracks session state changes
- macOS notifications to alert you when sessions need attention
- Zero configuration setup that works out of the box with sensible defaults

## Architecture

Following OpenAI Codex design patterns, cctmux is built as a workspace of independent crates:

```
cctmux/
├── protocol/    # Type definitions and IPC messages
├── core/        # Business logic (UI-agnostic)
├── db/          # SQLite persistence layer
├── tmux/        # tmux operation library (reusable)
├── tui/         # ratatui-based dashboard
├── notify/      # macOS notification system
├── hooks/       # Claude Code hooks integration
├── daemon/      # Background monitoring process
├── cli/         # CLI entry point
└── common/      # Shared utilities
```

## Installation

```bash
# Clone the repository
git clone https://github.com/ryo-morimoto/cctmux
cd cctmux

# Build from source
cargo build --release

# Install binaries
cargo install --path cctmux/cli
```

## Usage

### Basic Usage

```bash
# Launch TUI dashboard (default)
cctmux
```

### Keyboard Shortcuts (in TUI)

- `j/k` or `↑/↓`: Navigate pane list
- `Enter`: Jump to selected pane
- `w`: Jump to next waiting (approval-required) pane
- `e`: Jump to next error pane
- `r`: Jump to next running pane
- `q`: Quit dashboard

## How It Works

1. **Hook Integration**: Install Claude Code hooks to capture session events
2. **Daemon Monitoring**: Background process tracks tmux panes and session states
3. **Database Storage**: SQLite stores session history and current states
4. **TUI Display**: Dashboard shows real-time status with color-coded indicators
5. **Notifications**: macOS alerts notify you when approval is needed

## Development

### Prerequisites

- Rust 1.88 or later
- tmux 3.0 or later
- macOS (for notifications)

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Project Structure

See individual crate READMEs for detailed documentation:

- [protocol](cctmux/protocol/README.md) - Type definitions
- [core](cctmux/core/README.md) - Business logic
- [db](cctmux/db/README.md) - Database layer
- [tmux](cctmux/tmux/README.md) - tmux operations
- [tui](cctmux/tui/README.md) - TUI implementation
- [cli](cctmux/cli/README.md) - CLI entry point
- [daemon](cctmux/daemon/README.md) - Background daemon
- [hooks](cctmux/hooks/README.md) - Claude Hooks integration
- [notify](cctmux/notify/README.md) - Notification system

## Design Philosophy

cctmux follows OpenAI Codex architectural patterns:

- Separation of concerns where each crate has a single, well-defined responsibility
- Minimal dependencies with protocol layer containing zero business logic
- Reusability through tmux crate designed as standalone library
- Extension via traits where functionality is added through Extension traits
- TUI patterns using Renderable trait for composable UI components

## Roadmap

- [ ] Implement core session management logic
- [ ] Build tmux integration layer
- [ ] Create TUI dashboard with Renderable pattern
- [ ] Develop daemon monitoring loop
- [ ] Integrate Claude Code hooks
- [ ] Add macOS notification support
- [ ] Write comprehensive tests
- [ ] Create installation script

## Contributing

Contributions are welcome. Please follow these guidelines:

1. Read the [Architecture Documentation](docs/architecture.md)
2. Check existing issues or create a new one
3. Fork the repository and create a feature branch
4. Write tests for new functionality
5. Ensure `cargo test` and `cargo build` pass
6. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details

## Acknowledgments

- Inspired by [OpenAI Codex](https://github.com/openai/codex) architecture
- Built with [ratatui](https://github.com/ratatui-org/ratatui)
- Uses [clap](https://github.com/clap-rs/clap) for CLI parsing

---

**Status**: 🚧 Work in Progress - Initial structure complete, implementation phase starting
