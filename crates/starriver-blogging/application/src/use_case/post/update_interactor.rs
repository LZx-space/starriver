use starriver_blogging_domain::post::value_object::{Content, Title};
use starriver_shared_base::{
    authentication::principal::DefaultUser,
    cache::Cache,
    db::{Connection, Revision, Transaction},
    dto::OffsetPage,
};
use time::OffsetDateTime;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    dto::post_dto::{
        input::SaveOrUpdatePostCmd,
        output::{PostDetailDto, PostExcerptDto},
    },
    error::CtxError,
    port::{
        category_repository::CategoryRepository, post_cache::PostPageKey,
        post_repository::PostRepository,
    },
};

pub struct UpdateInteractor<Conn, PR, CR, PC, IC> {
    conn: Conn,
    post_repo: PR,
    category_repo: CR,
    page_cache: PC,
    item_cache: IC,
}

impl<Conn, PR, CR, PC, IC> UpdateInteractor<Conn, PR, CR, PC, IC>
where
    Conn: Connection,
    PR: PostRepository<Conn> + PostRepository<<Conn as Connection>::Transaction>,
    CR: CategoryRepository<Conn> + CategoryRepository<<Conn as Connection>::Transaction>,
    PC: Cache<PostPageKey, OffsetPage<PostExcerptDto>>,
    IC: Cache<Uuid, Option<PostDetailDto>>,
{
    /// 新建
    pub fn new(
        conn: Conn,
        post_repo: PR,
        category_repo: CR,
        page_cache: PC,
        item_cache: IC,
    ) -> Self {
        Self {
            conn,
            post_repo,
            category_repo,
            page_cache,
            item_cache,
        }
    }

    pub async fn update(
        &self,
        operator: DefaultUser,
        id: Uuid,
        cmd: SaveOrUpdatePostCmd,
    ) -> Result<(), CtxError> {
        info!(
            user_id = %operator.id,
            post_id = %id,
            "updating post"
        );

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

        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;
        let result = async {
            // 读-改-写必须留在同一事务内
            let post = self.post_repo.find_by_id(&tx, id).await?;
            let Some(mut found) = post else {
                return Err(CtxError::NotFound(format!("Post [{}] not found", id)));
            };

            let original = found.clone();

            let title = Title::new(cmd.title)?;
            let content = Content::new(cmd.content)?;
            found.update(title, content, cmd.category_id, cmd.attachments);
            if !cmd.publish {
                found.draft();
            } else {
                found.publish(OffsetDateTime::now_utc)?;
            }
            self.post_repo
                .update(&tx, Revision::new(original, found))
                .await
                .map_err(CtxError::from)
        }
        .await;

        match result {
            Ok(_) => {
                tx.commit().await.map_err(|e| {
                    error!(user_id=%operator.id, error=%e, "commit transaction failed");
                    CtxError::Internal
                })?;
                // 更新帖子后，清除所有帖子缓存
                self.page_cache.invalidate_all();
                self.item_cache.invalidate(&id).await;
                Ok(())
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
