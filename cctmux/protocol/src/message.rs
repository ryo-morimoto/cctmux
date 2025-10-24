/**
 * [実装意図]
 * daemon⇔TUI間のIPC通信メッセージ定義。
 * Unix domain socketやファイル経由での通信に使用。
 */

use crate::{PaneState, Session};
use serde::{Deserialize, Serialize};

/// daemon⇔TUI間の通信メッセージ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    /// 全セッション情報のリクエスト
    ListSessions,

    /// 全セッション情報のレスポンス
    SessionList(Vec<Session>),

    /// 特定paneの状態変更通知
    StateChanged {
        pane_id: String,
        new_state: PaneState,
    },

    /// エラー通知
    Error {
        message: String,
    },

    /// Ping（接続確認）
    Ping,

    /// Pong（接続確認応答）
    Pong,
}

impl Message {
    /// メッセージをJSONにシリアライズ
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// JSONからメッセージをデシリアライズ
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = Message::StateChanged {
            pane_id: "0:1.2".to_string(),
            new_state: PaneState::WaitingApproval,
        };

        let json = msg.to_json().unwrap();
        let deserialized = Message::from_json(&json).unwrap();

        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_ping_pong() {
        let ping = Message::Ping;
        let pong = Message::Pong;

        assert_ne!(ping, pong);

        let ping_json = ping.to_json().unwrap();
        let pong_json = pong.to_json().unwrap();

        assert_eq!(Message::from_json(&ping_json).unwrap(), Message::Ping);
        assert_eq!(Message::from_json(&pong_json).unwrap(), Message::Pong);
    }
}
