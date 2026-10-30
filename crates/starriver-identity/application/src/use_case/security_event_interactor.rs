use starriver_shared_base::{db::Connection, dto::CursorPage};

use crate::{
    dto::user_dto::{input::SecurityEventQuery, output::SecurityEventDto},
    error::CtxError,
    port::security_event_port::SecurityEventPort,
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

    pub async fn paginate(
        &self,
        q: SecurityEventQuery,
    ) -> Result<CursorPage<SecurityEventDto>, CtxError> {
        self.security_event_port
            .paginate(&self.conn, q)
            .await
            .map_err(|e| CtxError::internal("list security events failed", e))
    }
}
