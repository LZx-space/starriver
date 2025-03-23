use crate::repository::blog::blog_repository::BlogRepositoryImpl;
use sea_orm::DatabaseConnection;
use stariver_domain::blog::entity::Blog;
use stariver_domain::blog::repository::BlogRepository;
use stariver_infrastructure::model::blog::BlogPreview;
use stariver_infrastructure::model::err::CodedErr;
use stariver_infrastructure::model::page::{PageQuery, PageResult};
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

    pub async fn page(&self, q: PageQuery) -> Result<PageResult<BlogPreview>, CodedErr> {
        self.repo
            .find_page(q)
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, CodedErr> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }

    pub async fn add(&self, e: Blog) -> Result<Blog, CodedErr> {
        self.repo
            .add(e)
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, CodedErr> {
        self.repo
            .delete_by_id(id)
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }

    pub async fn update(&self, e: Blog) -> Result<Option<Blog>, CodedErr> {
        self.repo
            .update(e)
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }
}
