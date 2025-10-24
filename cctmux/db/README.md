# cctmux-db

**Data persistence layer**

## Design Philosophy

This crate handles all database operations using SQLite for persistent storage of Claude Code session information.

## Responsibilities

- SQLite database connection and migration management
- CRUD operations for sessions
- Query building and execution
- Data model conversions between protocol types and database rows

## Dependencies

**Internal crates:**
- `cctmux-protocol` - Session and state types

**External dependencies:**
- `rusqlite` - SQLite database interface (bundled feature enabled)
- `anyhow` - Error handling
- `thiserror` - Custom error types

## Usage Example

```rust
use cctmux_db::Database;
use cctmux_protocol::{Session, PaneState};

let db = Database::open("~/.local/share/cctmux/sessions.db")?;
db.create_session(&session)?;
let all_sessions = db.list_sessions()?;
db.update_state("0:1.2", PaneState::Completed)?;
```

## Schema Design

The database uses a simple schema optimized for fast queries:
- Sessions table with pane_id as primary key
- Indexes on state for filtering active sessions
- JSON storage for extensibility

## Architecture Notes

Following best practices for SQLite in Rust applications with bundled SQLite for zero system dependencies.
