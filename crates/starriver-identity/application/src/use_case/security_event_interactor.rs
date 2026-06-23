use starriver_shared_base::{
    db::Connection,
    dto::{PageQuery, PageResult},
};
use tracing::error;

use crate::{
    dto::user_dto::res::SecurityEventDto, error::CtxError,
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

    pub async fn paginate(&self, q: PageQuery) -> Result<PageResult<SecurityEventDto>, CtxError> {
        self.security_event_port
            .paginate(&self.conn, q)
            .await
            .map_err(|e| {
                error!(error=%e, "paginate users failed");
                CtxError::Internal
            })
    }
}
