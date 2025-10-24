# cctmux-notify

**macOS notification system**

## Design Philosophy

Platform-specific notification handling for macOS. Sends notifications to the macOS Notification Center when Claude Code sessions require user attention.

## Responsibilities

- Send notifications for approval-required events
- Send notifications for error events
- Notification click handling to focus tmux pane

## Dependencies

**Internal crates:**
- `cctmux-protocol` - Session and state types

**External dependencies:**
- `notify-rust` - Cross-platform notification library
- `anyhow` - Error handling

## Usage Example

```rust
use cctmux_notify::send_notification;
use cctmux_protocol::PaneState;

send_notification(
    "Claude Code Needs Approval",
    "Session in pane 0:1.2 is waiting for your approval",
    "0:1.2"
)?;
```

## Architecture Notes

Currently macOS-only via `notify-rust`. Future expansion could support Linux (libnotify) and Windows (Windows Notification API) if needed.
