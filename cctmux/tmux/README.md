# cctmux-tmux

**tmux operation library**

## Design Philosophy

An independent library for tmux operations that could be reused in other projects. Provides a Rust API wrapper around tmux commands.

## Responsibilities

- Pane identification and management
- tmux session operations
- Command execution (list-panes, select-pane, etc.)
- tmux output parsing

## Dependencies

**Internal crates:**
- `cctmux-protocol` - Pane and session types

**External dependencies:**
- `anyhow` - Error handling
- `thiserror` - Custom error types

## Usage Example

```rust
use cctmux_tmux::{Pane, TmuxSession};

// List all panes in current session
let panes = Pane::list_all()?;

// Jump to specific pane
Pane::select("0:1.2")?;

// Get current pane info
let current = Pane::current()?;
println!("Current pane: {}", current.id);
```

## Architecture Notes

This crate is designed as a standalone library with minimal dependencies, making it suitable for use in other tmux-related Rust projects. It follows the Codex pattern of creating reusable, focused libraries.
