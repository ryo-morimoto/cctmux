# cctmux-daemon

**Background daemon process**

## Design Philosophy

Long-running background process that monitors tmux sessions and maintains state. Automatically started when needed.

## Responsibilities

- Monitor tmux panes for Claude Code sessions
- Detect state changes through hooks
- Send notifications for events requiring user attention
- IPC communication with TUI for real-time updates
- Lifecycle management (auto-start, graceful shutdown)

## Dependencies

**Internal crates:**
- `cctmux-protocol` - Session and state types
- `cctmux-core` - Business logic
- `cctmux-db` - State persistence
- `cctmux-tmux` - Pane monitoring
- `cctmux-notify` - Notification sending
- `cctmux-hooks` - Hook event processing

**External dependencies:**
- `tokio` - Async runtime
- `anyhow` - Error handling
- `tracing` - Structured logging
- `tracing-subscriber` - Log formatting

## Binary

This crate produces the `cctmux-daemon` binary, which runs in the background.

## Auto-start Mechanism

The daemon is automatically started by the CLI when needed. It checks if a daemon is already running before spawning a new instance.

## Architecture Notes

The daemon acts as the central coordinator, receiving events from hooks, updating the database, sending notifications, and serving state information to the TUI.
