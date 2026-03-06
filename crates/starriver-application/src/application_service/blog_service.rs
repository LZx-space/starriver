use crate::assembler::blog_assembler::{cmd_2_new_entity, cmd_2_update_entity, entity_2_vo};
use crate::dto::blog_dto::{BlogCmd, BlogDetail, BlogSummary};
use crate::query::blog_query_service::{BlogQueryService, DefaultBlogQueryService};
use crate::repository::blog_repository::DefaultBlogRepository;
use sea_orm::DatabaseConnection;
use starriver_domain::blog::entity::Blog;
use starriver_domain::blog::repository::BlogRepository;
use starriver_infrastructure::error::error::{ApiError, Cause};
use starriver_infrastructure::model::page::{PageQuery, PageResult};
use uuid::Uuid;

pub struct BlogApplication {
    repo: DefaultBlogRepository,
    query_service: DefaultBlogQueryService,
}

impl BlogApplication {
    /// 新建
    pub fn new(conn: &'static DatabaseConnection) -> BlogApplication {
        BlogApplication {
            repo: DefaultBlogRepository { conn },
            query_service: DefaultBlogQueryService { conn },
        }
    }

    pub async fn page(&self, q: PageQuery) -> Result<PageResult<BlogSummary>, ApiError> {
        self.query_service.find_page(q).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<BlogDetail, ApiError> {
        self.find_entity_by_id(id).await.map(entity_2_vo)
    }

    pub async fn add(&self, cmd: BlogCmd) -> Result<BlogDetail, ApiError> {
        let blog = cmd_2_new_entity(cmd, "LZx".to_string());
        self.repo.add(blog).await.map(entity_2_vo)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, ApiError> {
        self.repo.delete_by_id(id).await
    }

    pub async fn update(&self, id: Uuid, cmd: BlogCmd) -> Result<BlogDetail, ApiError> {
        let existing_blog = self.find_entity_by_id(id).await?;
        let updated_blog = cmd_2_update_entity(cmd, existing_blog);
        self.repo
            .update(updated_blog)
            .await?
            .map(entity_2_vo)
            .ok_or_else(|| ApiError::new(Cause::DbError, format!("更新博客(id={})失败", id)))
    }

    // private-------------------------------------------------------------------
    async fn find_entity_by_id(&self, id: Uuid) -> Result<Blog, ApiError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| ApiError::new(Cause::ClientBadRequest, format!("博客(id={})不存在", id)))
    }
}
