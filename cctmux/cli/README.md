# cctmux-cli

**CLI entry point**

## Design Philosophy

Following Codex's CLI architecture: the entry point handles argument parsing and dispatches to appropriate components (TUI by default).

## Responsibilities

- Command-line argument parsing (using clap)
- `--help` and `--version` handling
- Dispatching to TUI
- Error message formatting
- Exit code management
- Daemon auto-start orchestration

## Dependencies

**Internal crates:**
- `cctmux-protocol` - Type definitions
- `cctmux-tui` - TUI library

**External dependencies:**
- `clap` - CLI parsing (derive feature)
- `anyhow` - Error handling

## Binary

This crate produces the `cctmux` binary, the main user-facing command.

## Usage

```bash
# Launch TUI dashboard (default)
cctmux

# Show help
cctmux --help

# Show version
cctmux --version
```

## Architecture Notes

Following Codex's MultitoolCli pattern:
- Default behavior with no subcommands launches TUI
- CLI layer separated from TUI implementation for clean separation of concerns
- TUI crate exposed as library for reusability

The CLI orchestrates daemon auto-start before launching the TUI, ensuring the background monitoring is always active.
