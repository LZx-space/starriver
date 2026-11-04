use starriver_shared_base::{
    authentication::principal::DefaultUser,
    cache::Cache,
    db::{Connection, Transaction},
    dto::OffsetPage,
};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    dto::post_dto::output::{PostDetailDto, PostExcerptDto},
    error::CtxError,
    port::{post_cache::PostPageKey, post_repository::PostRepository},
};

pub struct DeleteInteractor<Conn, PR, PC, IC> {
    conn: Conn,
    post_repo: PR,
    page_cache: PC,
    item_cache: IC,
}

impl<Conn, PR, PC, IC> DeleteInteractor<Conn, PR, PC, IC>
where
    Conn: Connection,
    PR: PostRepository<Conn> + PostRepository<<Conn as Connection>::Transaction>,
    PC: Cache<PostPageKey, OffsetPage<PostExcerptDto>>,
    IC: Cache<Uuid, Option<PostDetailDto>>,
{
    /// 新建
    pub fn new(conn: Conn, post_repo: PR, page_cache: PC, item_cache: IC) -> Self {
        Self {
            conn,
            post_repo,
            page_cache,
            item_cache,
        }
    }

    pub async fn delete_by_id(&self, operator: DefaultUser, id: Uuid) -> Result<bool, CtxError> {
        info!(
            user_id = %operator.id,
            post_id = %id,
            "deleting post"
        );

        // 帖子与附件关联的删除必须同属一个事务
        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;

        let result = self.post_repo.delete(&tx, id).await.map_err(CtxError::from);

        match result {
            Ok(deleted) => {
                tx.commit().await.map_err(|e| {
                    error!(user_id=%operator.id, error=%e, "commit transaction failed");
                    CtxError::Internal
                })?;
                if deleted {
                    self.page_cache.invalidate_all();
                    self.item_cache.invalidate(&id).await;
                }
                Ok(deleted)
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    error!(user_id=%operator.id, error=%e, "rollback transaction failed");
                    CtxError::Internal
                })?;
                Err(e)
            }
        }
    }
}
