use starriver_blogging_domain::post::{entity::Post, params::PostUpdate, value_object::PostState};
use starriver_shared_base::{
    authentication::principal::DefaultUser,
    cache::Cache,
    db::{Connection, Revision, Transaction},
    dto::{PageResult, PageSearch},
};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    dto::post_dto::{
        req::{PageQuery, SaveOrUpdatePostCmd},
        res::{PostDetailDto, PostExcerptDto, PostSearchDto},
    },
    error::CtxError,
    port::{
        category_repository::CategoryRepository,
        post_cache::{PostCaches, PostPageKey},
        post_query::PostQuery,
        post_repository::PostRepository,
    },
};

pub struct PostInteractor<Conn, Q, PR, CR, PC, DC> {
    conn: Conn,
    query: Q,
    post_repo: PR,
    category_repo: CR,
    cache: PostCaches<PC, DC>,
}

impl<Conn, Q, PR, CR, PC, DC> PostInteractor<Conn, Q, PR, CR, PC, DC>
where
    Conn: Connection,
    Q: PostQuery<Conn>,
    PR: PostRepository<Conn> + PostRepository<<Conn as Connection>::Transaction>,
    CR: CategoryRepository<Conn> + CategoryRepository<<Conn as Connection>::Transaction>,
    PC: Cache<PostPageKey, PageResult<PostExcerptDto>>,
    DC: Cache<Uuid, Option<PostDetailDto>>,
{
    /// 新建
    pub fn new(
        conn: Conn,
        query: Q,
        post_repo: PR,
        category_repo: CR,
        cache: PostCaches<PC, DC>,
    ) -> Self {
        Self {
            conn,
            query,
            post_repo,
            category_repo,
            cache,
        }
    }

    pub async fn paginate(&self, q: PageQuery) -> Result<PageResult<PostExcerptDto>, CtxError> {
        let key = PostPageKey {
            page: q.page,
            page_size: q.page_size,
            published_only: q.published_only,
            category_id: q.category_id,
        };
        self.cache
            .page_cache()
            .try_get_with(key, async { self.query.paginate(&self.conn, q).await })
            .await
            .map_err(|e| CtxError::internal("paginate posts failed", e))
    }

    pub async fn search(&self, q: PageSearch) -> Result<PageResult<PostSearchDto>, CtxError> {
        self.query
            .search(&self.conn, q)
            .await
            .map_err(|e| CtxError::internal("search posts failed", e))
    }

    pub async fn find(&self, id: Uuid) -> Result<PostDetailDto, CtxError> {
        self.cache
            .detail_cache()
            .try_get_with(id, async { self.query.find_detail(&self.conn, id).await })
            .await
            .map_err(|e| CtxError::internal("find post failed", e))
            .and_then(|r| r.ok_or_else(|| CtxError::NotFound(format!("post [{}] not exist", id))))
    }

    pub async fn create(
        &self,
        author: DefaultUser,
        cmd: SaveOrUpdatePostCmd,
    ) -> Result<PostDetailDto, CtxError> {
        let author_id = author.id;
        let state = match cmd.publish {
            true => PostState::Published,
            false => PostState::Draft,
        };
        if !self
            .category_repo
            .exists_by_id(&self.conn, cmd.category_id)
            .await?
        {
            return Err(CtxError::NotFound(format!(
                "category [{}] not exist",
                cmd.category_id
            )));
        }

        let post = Post::new(
            cmd.title,
            cmd.content,
            state,
            author_id,
            cmd.category_id,
            cmd.attachments,
        )?;
        let created = self.post_repo.add(&self.conn, post).await?;

        // 新增帖子后，清除所有帖子缓存
        self.cache.invalidate_all();

        let post_id = created.id().to_owned();
        self.find(post_id).await
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
        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;
        let result = async {
            if !self
                .category_repo
                .exists_by_id(&tx, cmd.category_id)
                .await?
            {
                return Err(CtxError::NotFound(format!(
                    "category [{}] not exist",
                    cmd.category_id
                )));
            }
            let post = self.post_repo.find_by_id(&tx, id).await?;
            let Some(mut found) = post else {
                return Err(CtxError::NotFound(format!("post [{}] not exist", id)));
            };
            let cmd = PostUpdate {
                title: cmd.title,
                content: cmd.content,
                category_id: cmd.category_id,
                attachments: cmd.attachments,
                published: cmd.publish,
            };
            let original = found.clone();
            found.update(cmd)?;
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
                self.cache.invalidate_all();
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

    pub async fn delete_by_id(&self, operator: DefaultUser, id: Uuid) -> Result<bool, CtxError> {
        info!(
            user_id = %operator.id,
            post_id = %id,
            "deleting post"
        );
        self.post_repo
            .delete(&self.conn, id)
            .await
            .map_err(CtxError::from)
            .inspect(|_| {
                // 更新帖子后，清除所有帖子缓存
                self.cache.invalidate_all();
            })
    }
}
