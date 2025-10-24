//! Claude Code session state enumeration.
//!
//! This enum is used across TUI display, database storage, and daemon monitoring
//! to represent the current state of a Claude Code session.

use serde::{Deserialize, Serialize};

/// Claude Code session execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaneState {
    /// Currently running
    Running,
    /// Waiting for user approval
    WaitingApproval,
    /// Completed successfully
    Completed,
    /// Error occurred
    Error,
}

impl PaneState {
    /// Get string representation of the state
    pub fn as_str(&self) -> &'static str {
        match self {
            PaneState::Running => "running",
            PaneState::WaitingApproval => "waiting",
            PaneState::Completed => "completed",
            PaneState::Error => "error",
        }
    }

    /// Check if this state requires user action
    pub fn requires_user_action(&self) -> bool {
        matches!(self, PaneState::WaitingApproval | PaneState::Error)
    }
}

impl std::fmt::Display for PaneState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requires_user_action() {
        assert!(!PaneState::Running.requires_user_action());
        assert!(PaneState::WaitingApproval.requires_user_action());
        assert!(!PaneState::Completed.requires_user_action());
        assert!(PaneState::Error.requires_user_action());
    }

    #[test]
    fn test_display() {
        assert_eq!(PaneState::Running.to_string(), "running");
        assert_eq!(PaneState::WaitingApproval.to_string(), "waiting");
        assert_eq!(PaneState::Completed.to_string(), "completed");
        assert_eq!(PaneState::Error.to_string(), "error");
    }
}
