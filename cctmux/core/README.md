# cctmux-core

**Business logic layer**

## Design Philosophy

Following OpenAI Codex's `codex-core` principles:

1. **UI-agnostic business logic**: No dependencies on TUI, daemon, or CLI implementations
2. **Platform-independent**: Core logic works across all platforms
3. **Reusable library**: Designed to be used by various UI implementations

## Responsibilities

- Session management logic
- State machine for Claude Code session transitions
- Business rules and validation
- Core algorithms independent of I/O

## Dependencies

**Internal crates:**
- `cctmux-protocol` - Type definitions

**External dependencies:**
- `anyhow` - Error handling
- `thiserror` - Custom error types

## Usage Example

```rust
use cctmux_core::SessionManager;
use cctmux_protocol::{Session, PaneState};

let mut manager = SessionManager::new();
let session = manager.create_session("0:1.2".to_string())?;
manager.update_state(&session.pane_id, PaneState::WaitingApproval)?;
```

## Architecture Notes

This crate implements pure business logic without UI concerns. It's designed as a library that can be used by Rust-based UIs (TUI, daemon, CLI) following the Codex architecture pattern.
