use starriver_shared_base::db::Connection;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    dto::user_dto::{input::SecurityEventQuery, output::SecurityEventPage},
    error::CtxError,
    port::security_event_port::{SecurityEventCursor, SecurityEventFilter, SecurityEventPort},
};

pub struct SecurityEventInteractor<Conn, SEP> {
    conn: Conn,
    security_event_port: SEP,
}

impl<Conn, SEP> SecurityEventInteractor<Conn, SEP>
where
    Conn: Connection,
    SEP: SecurityEventPort<Conn> + Sync,
{
    pub fn new(conn: Conn, security_event_port: SEP) -> Self {
        Self {
            conn,
            security_event_port,
        }
    }

    pub async fn list(&self, q: SecurityEventQuery) -> Result<SecurityEventPage, CtxError> {
        let after = match q.pagination.cursor.as_deref() {
            Some(value) => Some(SecurityEventCursor::decode(value).ok_or_else(|| {
                CtxError::InvalidInput("invalid security event cursor".to_string())
            })?),
            None => None,
        };

        let from = parse_timestamp(q.from.as_deref(), "from")?;
        let to = parse_timestamp(q.to.as_deref(), "to")?;
        if let (Some(from), Some(to)) = (&from, &to)
            && from > to
        {
            return Err(CtxError::InvalidInput(
                "from must be earlier than or equal to to".to_string(),
            ));
        }

        let filter = SecurityEventFilter {
            limit: q.pagination.limit as u64,
            after,
            user_id: q.user_id,
            event_type: q.event_type,
            from,
            to,
        };

        self.security_event_port
            .list(&self.conn, filter)
            .await
            .map_err(|e| CtxError::internal("list security events failed", e))
    }
}

fn parse_timestamp(value: Option<&str>, field: &str) -> Result<Option<OffsetDateTime>, CtxError> {
    value
        .map(|value| {
            OffsetDateTime::parse(value, &Rfc3339).map_err(|_| {
                CtxError::InvalidInput(format!("invalid {field} timestamp, expected RFC3339"))
            })
        })
        .transpose()
}
