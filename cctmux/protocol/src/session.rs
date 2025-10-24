/**
 * [実装意図]
 * Claude Codeセッションを表す型定義。
 * tmux paneとClaude Code実行状態を紐付ける。
 */

use crate::state::PaneState;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Claude Codeセッション
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    /// tmux pane ID（例: "0:1.2"）
    pub pane_id: String,

    /// タスク名（ユーザーが指定、またはClaude Codeから抽出）
    pub task_name: Option<String>,

    /// 現在の状態
    pub state: PaneState,

    /// セッション開始時刻
    pub started_at: DateTime<Utc>,

    /// 最終更新時刻
    pub updated_at: DateTime<Utc>,

    /// エラーメッセージ（Error状態の場合）
    pub error_message: Option<String>,
}

impl Session {
    /// 新規セッションを作成
    pub fn new(pane_id: String) -> Self {
        let now = Utc::now();
        Self {
            pane_id,
            task_name: None,
            state: PaneState::Running,
            started_at: now,
            updated_at: now,
            error_message: None,
        }
    }

    /// 状態を更新
    pub fn update_state(&mut self, new_state: PaneState) {
        self.state = new_state;
        self.updated_at = Utc::now();
    }

    /// エラー状態に更新
    pub fn set_error(&mut self, message: String) {
        self.state = PaneState::Error;
        self.error_message = Some(message);
        self.updated_at = Utc::now();
    }

    /// タスク名を設定
    pub fn set_task_name(&mut self, name: String) {
        self.task_name = Some(name);
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session() {
        let session = Session::new("0:1.2".to_string());
        assert_eq!(session.pane_id, "0:1.2");
        assert_eq!(session.state, PaneState::Running);
        assert_eq!(session.task_name, None);
    }

    #[test]
    fn test_update_state() {
        let mut session = Session::new("0:1.2".to_string());
        let before = session.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));
        session.update_state(PaneState::WaitingApproval);

        assert_eq!(session.state, PaneState::WaitingApproval);
        assert!(session.updated_at > before);
    }

    #[test]
    fn test_set_error() {
        let mut session = Session::new("0:1.2".to_string());
        session.set_error("Test error".to_string());

        assert_eq!(session.state, PaneState::Error);
        assert_eq!(session.error_message, Some("Test error".to_string()));
    }
}
