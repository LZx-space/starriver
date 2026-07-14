use starriver_blogging_domain::post::{
    entity::Post,
    params::PostUpdate,
    value_object::{Content, PostState, Title},
};
use starriver_shared_base::{
    authentication::PrincipalClaims,
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
        post_cache::{PostCaches, PostPageKey},
        post_query::PostQuery,
        post_repository::PostRepository,
    },
};

pub struct PostInteractor<Conn, Q, R, PC, DC> {
    conn: Conn,
    query: Q,
    repo: R,
    cache: PostCaches<PC, DC>,
}

impl<Conn, Q, R, PC, DC> PostInteractor<Conn, Q, R, PC, DC>
where
    Conn: Connection,
    Q: PostQuery<Conn>,
    R: PostRepository<Conn> + PostRepository<<Conn as Connection>::Transaction>,
    PC: Cache<PostPageKey, PageResult<PostExcerptDto>>,
    DC: Cache<Uuid, Option<PostDetailDto>>,
{
    /// 新建
    pub fn new(conn: Conn, query: Q, repo: R, cache: PostCaches<PC, DC>) -> Self {
        Self {
            conn,
            query,
            repo,
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
            .map_err(|e| {
                error!(error=%e, "database error");
                CtxError::Internal
            })
    }

    pub async fn search(&self, q: PageSearch) -> Result<PageResult<PostSearchDto>, CtxError> {
        self.query.search(&self.conn, q).await.map_err(|e| {
            error!(error=%e, "database error");
            CtxError::Internal
        })
    }

    pub async fn find(&self, id: Uuid) -> Result<PostDetailDto, CtxError> {
        self.cache
            .detail_cache()
            .try_get_with(id, async { self.query.find_detail(&self.conn, id).await })
            .await
            .map_err(|e| {
                error!(error=%e, "database error");
                CtxError::Internal
            })
            .and_then(|r| r.ok_or_else(|| CtxError::NotFound(format!("post [{}] not exist", id))))
    }

    pub async fn create(
        &self,
        author: PrincipalClaims,
        cmd: SaveOrUpdatePostCmd,
    ) -> Result<PostDetailDto, CtxError> {
        let author_id = author.sub;
        let state = match cmd.publish {
            true => PostState::Published,
            false => PostState::Draft,
        };

        let post = Post::new(
            Title::new(cmd.title)?,
            Content::new(cmd.content)?,
            state,
            author_id,
            cmd.category_id,
            cmd.attachments,
        )?;
        let created = self.repo.add(&self.conn, post).await?;

        // 新增帖子后，清除所有帖子缓存
        self.cache.invalidate_all();

        let post_id = created.id().to_owned();
        self.find(post_id).await
    }

    pub async fn update(
        &self,
        operator: PrincipalClaims,
        id: Uuid,
        cmd: SaveOrUpdatePostCmd,
    ) -> Result<(), CtxError> {
        info!(
            user_id = %operator.sub,
            post_id = %id,
            "updating post"
        );
        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;
        let result = async {
            let post = self.repo.find_by_id(&self.conn, id).await?;
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
            self.repo
                .update(&self.conn, Revision::new(original, found))
                .await
                .map_err(CtxError::from)
        }
        .await;

        match result {
            Ok(_) => {
                tx.commit().await.map_err(|e| {
                    error!(user_id=%operator.sub, error=%e, "commit transaction failed");
                    CtxError::Internal
                })?;
                // 更新帖子后，清除所有帖子缓存
                self.cache.invalidate_all();
                Ok(())
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    error!(user_id=%operator.sub, error=%e, "rollback transaction failed");
                    CtxError::Internal
                })?;
                Err(e)
            }
        }
    }

    pub async fn delete_by_id(
        &self,
        operator: PrincipalClaims,
        id: Uuid,
    ) -> Result<bool, CtxError> {
        info!(
            user_id = %operator.sub,
            Post_id = %id,
            "deleting post"
        );
        self.repo
            .delete(&self.conn, id)
            .await
            .map_err(CtxError::from)
            .inspect(|_| {
                // 更新帖子后，清除所有帖子缓存
                self.cache.invalidate_all();
            })
    }
}
