use starriver_shared_base::{db::Connection, dto::OffsetPage};

use crate::{
    dto::post_dto::{input::SearchQuery, output::PostSearchDto},
    error::CtxError,
    port::post_query::PostQuery,
};

pub struct SearchInteractor<Conn, Q> {
    conn: Conn,
    query: Q,
}

impl<Conn, Q> SearchInteractor<Conn, Q>
where
    Conn: Connection,
    Q: PostQuery<Conn>,
{
    /// 新建
    pub fn new(conn: Conn, query: Q) -> Self {
        Self { conn, query }
    }

    pub async fn search(&self, q: SearchQuery) -> Result<OffsetPage<PostSearchDto>, CtxError> {
        self.query
            .search(&self.conn, q)
            .await
            .map_err(|e| CtxError::internal("search posts failed", e))
    }
}
