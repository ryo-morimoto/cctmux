//! Protocol layer type definitions.
//!
//! This crate defines the core types and contracts for communication between components.
//! Following Codex design principles:
//! - Minimal dependencies (serde, chrono only)
//! - No business logic
//! - Extension through traits in other crates

pub mod session;
pub mod state;
pub mod message;

pub use session::Session;
pub use state::PaneState;
pub use message::Message;
