# cctmux-hooks

**Claude Code Hooks integration**

## Design Philosophy

Handles events from Claude Code hooks scripts, processing them and updating the database accordingly.

## Responsibilities

- Parse hook event payloads
- Process hook events (on_start, on_approval_needed, on_complete, on_error)
- Update database with new session states
- Trigger notifications when appropriate

## Dependencies

**Internal crates:**
- `cctmux-protocol` - Session and state types
- `cctmux-db` - Database operations

**External dependencies:**
- `serde_json` - JSON parsing from hook scripts
- `anyhow` - Error handling

## Usage Example

```rust
use cctmux_hooks::handle_hook_event;

// Called by the hook script with JSON payload
let event_json = r#"{"type": "approval_needed", "pane_id": "0:1.2"}"#;
handle_hook_event(event_json)?;
```

## Hook Script Integration

The `hooks/cctmux-hook.sh` script is installed in `~/.config/claude/hooks/` and calls this library's handler when Claude Code events occur.

## Architecture Notes

This crate acts as the bridge between Claude Code's hook system and cctmux's state management, ensuring all session state changes are captured and persisted.
