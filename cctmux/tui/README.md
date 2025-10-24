# cctmux-tui

**TUI dashboard**

## Design Philosophy

Interactive terminal UI built with ratatui, following Codex's rendering architecture patterns.

## Responsibilities

- Real-time dashboard display of all Claude Code sessions
- Keyboard-driven navigation and pane jumping
- Renderable trait implementation for composable UI components
- Event handling (keyboard input, resize events)

## Dependencies

**Internal crates:**
- `cctmux-protocol` - Session and state types
- `cctmux-core` - Business logic
- `cctmux-tmux` - Pane operations
- `cctmux-db` - Session data retrieval

**External dependencies:**
- `ratatui` - TUI framework
- `crossterm` - Terminal manipulation
- `anyhow` - Error handling

## Usage Example

```rust
use cctmux_tui;

// Run the TUI dashboard
cctmux_tui::run()?;
```

## Keyboard Shortcuts

- `j/k` or `↑/↓`: Navigate pane list
- `Enter`: Jump to selected pane
- `w`: Jump to next waiting (approval-required) pane
- `e`: Jump to next error pane
- `r`: Jump to next running pane
- `q`: Quit dashboard

## Architecture Notes

Following Codex's TUI patterns:
- **Renderable trait** for composable UI components
- **Color scheme**: cyan for selections, green for success, red for errors, magenta for branding
- **Style guide** compliance (no custom colors, ANSI colors only)
