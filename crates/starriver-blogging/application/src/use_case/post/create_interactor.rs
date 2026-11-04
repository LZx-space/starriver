use starriver_blogging_domain::post::{
    entity::Post,
    value_object::{Content, Title},
};
use starriver_shared_base::{
    authentication::principal::DefaultUser,
    cache::Cache,
    db::{Connection, Transaction},
    dto::OffsetPage,
};
use time::OffsetDateTime;
use tracing::error;

use crate::{
    dto::post_dto::{
        input::SaveOrUpdatePostCmd,
        output::{PostDetailDto, PostExcerptDto},
    },
    error::CtxError,
    port::{
        category_repository::CategoryRepository, post_cache::PostPageKey, post_query::PostQuery,
        post_repository::PostRepository,
    },
};

pub struct CreateInteractor<Conn, Q, PR, CR, PC> {
    conn: Conn,
    query: Q,
    post_repo: PR,
    category_repo: CR,
    page_cache: PC,
}

impl<Conn, Q, PR, CR, PC> CreateInteractor<Conn, Q, PR, CR, PC>
where
    Conn: Connection,
    Q: PostQuery<Conn>,
    PR: PostRepository<Conn> + PostRepository<<Conn as Connection>::Transaction>,
    CR: CategoryRepository<Conn> + CategoryRepository<<Conn as Connection>::Transaction>,
    PC: Cache<PostPageKey, OffsetPage<PostExcerptDto>>,
{
    /// 新建
    pub fn new(conn: Conn, query: Q, post_repo: PR, category_repo: CR, page_cache: PC) -> Self {
        Self {
            conn,
            query,
            post_repo,
            category_repo,
            page_cache,
        }
    }

    pub async fn create(
        &self,
        author: DefaultUser,
        cmd: SaveOrUpdatePostCmd,
    ) -> Result<PostDetailDto, CtxError> {
        if !self
            .category_repo
            .exists_by_id(&self.conn, cmd.category_id)
            .await?
        {
            return Err(CtxError::NotFound(format!(
                "Category [{}] not found",
                cmd.category_id
            )));
        }

        let title = Title::new(cmd.title)?;
        let content = Content::new(cmd.content)?;
        let mut post = Post::new(title, content, author.id, cmd.category_id, cmd.attachments);
        if cmd.publish {
            post.publish(OffsetDateTime::now_utc)?;
        }

        // post_repo.add() 包含post与附件关联的写入，需事务
        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;
        let result = self.post_repo.add(&tx, post).await.map_err(CtxError::from);

        match result {
            Ok(created) => {
                tx.commit().await.map_err(|e| {
                    error!(error = %e, "commit transaction failed");
                    CtxError::Internal
                })?;
                // 提交成功后，清除帖子分页缓存
                self.page_cache.invalidate_all();
                let (id, ..) = created.into_parts();
                self.query
                    .find_detail(&self.conn, id)
                    .await
                    .map_err(|e| CtxError::internal("find post failed", e))?
                    .ok_or_else(|| CtxError::NotFound(format!("Post [{}] not found", id)))
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    error!(error = %e, "rollback transaction failed");
                    CtxError::Internal
                })?;
                Err(e)
            }
        }
    }
}
