use std::convert::Infallible;

use starriver_shared_base::{db::Executor, error::QueryError};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dto::user_dto::{
    input::{SecurityEventCmd, SecurityEventType},
    output::SecurityEventPage,
};

/// 游标：按 `(created_at, id)` 降序定位上一页最后一条事件。
#[derive(Clone, Debug)]
pub struct SecurityEventCursor {
    pub occurred_at: OffsetDateTime,
    pub id: Uuid,
}

impl SecurityEventCursor {
    pub fn encode(&self) -> String {
        format!("{}:{}", self.occurred_at.unix_timestamp_nanos(), self.id)
    }

    pub fn decode(value: &str) -> Option<Self> {
        let (nanos, id) = value.split_once(':')?;
        let occurred_at = OffsetDateTime::from_unix_timestamp_nanos(nanos.parse().ok()?).ok()?;
        let id = Uuid::parse_str(id).ok()?;
        Some(Self { occurred_at, id })
    }
}

pub struct SecurityEventFilter {
    pub limit: u64,
    pub after: Option<SecurityEventCursor>,
    pub user_id: Option<Uuid>,
    pub event_type: Option<SecurityEventType>,
    pub from: Option<OffsetDateTime>,
    pub to: Option<OffsetDateTime>,
}

/// SecurityEvent无业务性，仅保存查询，所以这里不是Repository，简单的面向数据
pub trait SecurityEventPort<T: Executor> {
    fn list(
        &self,
        conn: &T,
        filter: SecurityEventFilter,
    ) -> impl Future<Output = Result<SecurityEventPage, QueryError>> + Send;

    fn insert(
        &self,
        conn: &T,
        event: SecurityEventCmd,
    ) -> impl Future<Output = Result<(), Infallible>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_round_trips() {
        let cursor = SecurityEventCursor {
            occurred_at: OffsetDateTime::now_utc(),
            id: Uuid::now_v7(),
        };
        let decoded = SecurityEventCursor::decode(&cursor.encode()).expect("valid cursor");
        assert_eq!(decoded.occurred_at, cursor.occurred_at);
        assert_eq!(decoded.id, cursor.id);
    }

    #[test]
    fn invalid_cursor_is_rejected() {
        assert!(SecurityEventCursor::decode("not-a-cursor").is_none());
        assert!(SecurityEventCursor::decode("123:not-a-uuid").is_none());
    }
}
