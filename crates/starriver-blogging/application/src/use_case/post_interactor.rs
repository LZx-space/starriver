use sea_orm::ConnectionTrait;
use starriver_blogging_domain::post::{
    entity::Post,
    params::PostUpdate,
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
    port::{post_query::PostQuery, post_repository::PostRepository},
};

pub struct PostApplication<Conn, Q, PR> {
    conn: Conn,
    query: Q,
    repo: PR,
}

impl<Conn, Q, PR> PostApplication<Conn, Q, PR>
where
    Conn: ConnectionTrait,
    Q: PostQuery,
    PR: PostRepository,
{
    /// 新建
    pub fn new(conn: Conn, query: Q, repo: PR) -> Self {
        Self { conn, query, repo }
    }

    pub async fn paginate(&self, q: PageQuery) -> Result<PageResult<PostExcerptDto>, CtxError> {
        self.query
            .paginate(&self.conn, q)
            .await
            .map_err(CtxError::from)
    }

    pub async fn find(&self, id: Uuid) -> Result<PostDetailDto, CtxError> {
        self.query
            .find_detail(&self.conn, id)
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
        let created = self.repo.add(&self.conn, post).await?;
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
            .find_by_id(&self.conn, id)
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
        self.repo
            .update(&self.conn, Revision::new(original, found))
            .await?;
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
        self.repo.delete(&self.conn, id).await.map(Ok)?
    }
}
