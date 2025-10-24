# cctmux-protocol

**Type definitions and protocol layer**

## Design Philosophy

Following OpenAI Codex's `codex-protocol` principles:

1. **Minimal dependencies**: Only essential dependencies (serde, chrono)
2. **No business logic**: Type definitions and conversions only
3. **Extension through traits**: Add functionality via Extension traits in other crates

## Responsibilities

- Claude Code session type definitions
- Pane state enumeration
- IPC message definitions for daemon⇔TUI communication
- Trait definitions for database model conversions (implementations in respective crates)

## Dependencies

**External dependencies:**
- `serde` - Serialization/deserialization
- `serde_json` - JSON support
- `chrono` - Date/time handling

## Usage Example

```rust
use cctmux_protocol::{Session, PaneState};

let session = Session {
    pane_id: "0:1.2".to_string(),
    task_name: Some("API implementation".to_string()),
    state: PaneState::Running,
    started_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
    error_message: None,
};
```

## Architecture Notes

This crate establishes the contract layer between components, ensuring standardized communication without heavy dependencies. Types defined here are shared across all other crates to maintain consistency.
