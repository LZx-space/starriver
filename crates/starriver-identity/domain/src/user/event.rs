use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct UserSecurityEvent {
    user_id: Uuid,
    username: String,
    kind: UserSecurityEventKind,
    occurred_at: OffsetDateTime,
    payload: Option<serde_json::Value>,
}

impl UserSecurityEvent {
    pub fn new(
        user_id: Uuid,
        username: String,
        kind: UserSecurityEventKind,
        occurred_at: OffsetDateTime,
        payload: Option<serde_json::Value>,
    ) -> Self {
        Self {
            user_id,
            username,
            kind,
            occurred_at,
            payload,
        }
    }

    pub fn into_parts(
        self,
    ) -> (
        Uuid,
        String,
        UserSecurityEventKind,
        OffsetDateTime,
        Option<serde_json::Value>,
    ) {
        (
            self.user_id,
            self.username,
            self.kind,
            self.occurred_at,
            self.payload,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserSecurityEventKind {
    LoginSucceeded,
    LoginWithBadPassword,
    UserLocked,
    PasswordChanged,
    PasswordReset,
}
