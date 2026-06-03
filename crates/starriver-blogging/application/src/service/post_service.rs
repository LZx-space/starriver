use starriver_blogging_domain::post::{
    entity::Post,
    params::PostUpdate,
    repository::PostRepository,
    value_object::{Content, PostState, Title},
};
use starriver_shared_base::{
    authentication::PrincipalClaims, dto::PageResult, repository::Revision,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    dto::post_dto::{
        req::{PageQuery, SaveOrUpdatePostCmd},
        res::{PostDetailDto, PostExcerptDto},
    },
    error::CtxError,
    port_out::post_query_port::PostQueryPort,
};

pub struct PostApplication<Q, PR> {
    query: Q,
    repo: PR,
}

impl<Q, PR> PostApplication<Q, PR>
where
    Q: PostQueryPort,
    PR: PostRepository,
{
    /// 新建
    pub fn new(query: Q, repo: PR) -> Self {
        Self { query, repo }
    }

    pub async fn paginate(&self, q: PageQuery) -> Result<PageResult<PostExcerptDto>, CtxError> {
        self.query.paginate(q).await.map_err(CtxError::from)
    }

    pub async fn find(&self, id: Uuid) -> Result<PostDetailDto, CtxError> {
        self.query
            .find_detail(id)
            .await?
            .ok_or_else(|| CtxError::NotFound(format!("post [{}] not exist", id)))
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
        let created = self.repo.add(post).await?;
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
        let mut found = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| CtxError::NotFound(format!("post [{}] not exist", id)))?;
        let cmd = PostUpdate {
            title: cmd.title,
            content: cmd.content,
            category_id: cmd.category_id,
            attachments: cmd.attachments,
            published: cmd.publish,
        };
        let original = found.clone();
        found.update(cmd)?;
        self.repo.update(Revision::new(original, found)).await?;
        Ok(())
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
        self.repo.delete(id).await.map(Ok)?
    }
}
