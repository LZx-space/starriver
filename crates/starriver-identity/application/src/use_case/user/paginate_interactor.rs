use starriver_shared_base::{
    db::Connection,
    dto::{OffsetPage, OffsetPageQuery},
};

use crate::{dto::user_dto::output::UserDetailDto, error::CtxError, port::user_query::UserQuery};

pub struct PaginateInteractor<Conn, UQ> {
    conn: Conn,
    user_query: UQ,
}

impl<Conn, UQ> PaginateInteractor<Conn, UQ>
where
    Conn: Connection,
    UQ: UserQuery<Conn> + Sync,
{
    pub fn new(conn: Conn, user_query: UQ) -> Self {
        Self { conn, user_query }
    }

    pub async fn paginate(
        &self,
        q: OffsetPageQuery,
    ) -> Result<OffsetPage<UserDetailDto>, CtxError> {
        self.user_query
            .paginate(&self.conn, q)
            .await
            .map_err(|e| CtxError::internal("paginate users failed", e))
    }
}
