use starriver_blogging_domain::post::{
    entity::Post, params::PostUpdate, repository::PostRepository,
};
use starriver_shared_base::{
    authentication::PrincipalClaims, dto::PageResult, repository::Revision,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    dto::post_dto::{
        req::{PageQuery, UpdatePostCmd},
        res::{PostDetail, PostExcerpt},
    },
    error::CtxError,
    port_out::post_query_port::PostQueryPort,
};

pub struct PostApplication<Q, R> {
    query: Q,
    repo: R,
}

impl<Q, R> PostApplication<Q, R>
where
    Q: PostQueryPort,
    R: PostRepository,
{
    /// 新建
    pub fn new(query: Q, repo: R) -> Self {
        Self { query, repo }
    }

    pub async fn paginate(&self, q: PageQuery) -> Result<PageResult<PostExcerpt>, CtxError> {
        self.query.paginate(q).await.map_err(CtxError::from)
    }

    pub async fn find(&self, id: Uuid) -> Result<PostDetail, CtxError> {
        let post = self
            .query
            .find_detail(id)
            .await?
            .ok_or_else(|| CtxError::NotFound(format!("post [{}] not exist", id)))?;
        Ok(post)
    }

    pub async fn create_draft(&self, author: PrincipalClaims) -> Result<PostDetail, CtxError> {
        let author_id = author.sub;
        let draft_post = Post::new_empty_draft(author_id)?;
        let created = self.repo.add(draft_post).await?;
        let post_id = created.id().to_owned();
        self.find(post_id).await
    }

    pub async fn update(
        &self,
        operator: PrincipalClaims,
        id: Uuid,
        cmd: UpdatePostCmd,
    ) -> Result<(), CtxError> {
        info!(
            user_id = %operator.sub,
            post_id = %id,
            "updating post"
        );
        let mut found = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| CtxError::NotFound(format!("post [{}] not exist", id)))?;
        let cmd = PostUpdate {
            title: cmd.title,
            content: cmd.content,
            category_id: cmd.category_id,
            attachment_ids: cmd.attachment_ids,
            published: cmd.publish,
        };
        let original = found.clone();
        found.update(cmd)?;
        self.repo
            .update(Revision::new(original, found))
            .await
            .map(|_| ())
            .map_err(CtxError::from)
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
        self.repo.delete_by_id(id).await.map_err(CtxError::from)
    }
}
