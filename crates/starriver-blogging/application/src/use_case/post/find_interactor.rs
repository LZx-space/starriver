use starriver_shared_base::{cache::Cache, db::Connection};
use uuid::Uuid;

use crate::{dto::post_dto::output::PostDetailDto, error::CtxError, port::post_query::PostQuery};

pub struct FindInteractor<Conn, Q, IC> {
    conn: Conn,
    query: Q,
    item_cache: IC,
}

impl<Conn, Q, IC> FindInteractor<Conn, Q, IC>
where
    Conn: Connection,
    Q: PostQuery<Conn>,
    IC: Cache<Uuid, Option<PostDetailDto>>,
{
    /// 新建
    pub fn new(conn: Conn, query: Q, item_cache: IC) -> Self {
        Self {
            conn,
            query,
            item_cache,
        }
    }

    pub async fn find(&self, id: Uuid) -> Result<PostDetailDto, CtxError> {
        self.item_cache
            .try_get_with(id, async { self.query.find_detail(&self.conn, id).await })
            .await
            .map_err(|e| CtxError::internal("find post failed", e))
            .and_then(|r| r.ok_or_else(|| CtxError::NotFound(format!("Post [{}] not found", id))))
    }
}
