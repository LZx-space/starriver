use crate::repository::blog::blog_repository::BlogRepositoryImpl;
use sea_orm::DatabaseConnection;
use starriver_domain::blog::entity::Blog;
use starriver_domain::blog::repository::BlogRepository;
use starriver_infrastructure::error::error::{ApiError, Cause};
use starriver_infrastructure::model::blog::BlogPreview;
use starriver_infrastructure::model::page::{PageQuery, PageResult};
use uuid::Uuid;

pub struct BlogApplication {
    repo: BlogRepositoryImpl,
}

impl BlogApplication {
    /// 新建
    pub fn new(conn: &'static DatabaseConnection) -> BlogApplication {
        BlogApplication {
            repo: BlogRepositoryImpl { conn },
        }
    }

    pub async fn page(&self, q: PageQuery) -> Result<PageResult<BlogPreview>, ApiError> {
        self.repo
            .find_page(q)
            .await
            .map_err(|e| ApiError::new(Cause::DbError, e.to_string()))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, ApiError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::new(Cause::DbError, e.to_string()))
    }

    pub async fn add(&self, e: Blog) -> Result<Blog, ApiError> {
        self.repo
            .add(e)
            .await
            .map_err(|e| ApiError::new(Cause::DbError, e.to_string()))
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, ApiError> {
        self.repo
            .delete_by_id(id)
            .await
            .map_err(|e| ApiError::new(Cause::DbError, e.to_string()))
    }

    pub async fn update(&self, e: Blog) -> Result<Option<Blog>, ApiError> {
        self.repo
            .update(e)
            .await
            .map_err(|e| ApiError::new(Cause::DbError, e.to_string()))
    }
}
