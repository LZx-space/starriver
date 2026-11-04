use starriver_shared_base::{cache::Cache, db::Connection, dto::OffsetPage};

use crate::{
    dto::post_dto::{input::PageQuery, output::PostExcerptDto},
    error::CtxError,
    port::{post_cache::PostPageKey, post_query::PostQuery},
};

pub struct PaginateInteractor<Conn, Q, PC> {
    conn: Conn,
    query: Q,
    page_cache: PC,
}

impl<Conn, Q, PC> PaginateInteractor<Conn, Q, PC>
where
    Conn: Connection,
    Q: PostQuery<Conn>,
    PC: Cache<PostPageKey, OffsetPage<PostExcerptDto>>,
{
    /// 新建
    pub fn new(conn: Conn, query: Q, page_cache: PC) -> Self {
        Self {
            conn,
            query,
            page_cache,
        }
    }

    pub const CACHE_PAGE_LIMIT: u64 = 3;

    pub async fn paginate(&self, q: PageQuery) -> Result<OffsetPage<PostExcerptDto>, CtxError> {
        if q.pagination.page >= Self::CACHE_PAGE_LIMIT {
            self.query
                .paginate(&self.conn, q)
                .await
                .map_err(|e| CtxError::internal("paginate posts failed", e))
        } else {
            let key = PostPageKey {
                page: q.pagination.page,
                page_size: q.pagination.page_size,
                published_only: q.published_only,
                category_id: q.category_id,
            };
            self.page_cache
                .try_get_with(key, async { self.query.paginate(&self.conn, q).await })
                .await
                .map_err(|e| CtxError::internal("paginate posts failed", e))
        }
    }
}
