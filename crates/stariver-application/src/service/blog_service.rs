use crate::repository::blog::blog_repository::ArticleRepositoryImpl;
use sea_orm::DatabaseConnection;
use stariver_domain::blog::aggregate::Article;
use stariver_domain::blog::repository::ArticleRepository;
use stariver_infrastructure::model::blog::ArticleSummary;
use stariver_infrastructure::model::err::CodedErr;
use stariver_infrastructure::model::page::{PageQuery, PageResult};
use uuid::Uuid;

pub struct ArticleApplication {
    repo: ArticleRepositoryImpl,
}

impl ArticleApplication {
    /// 新建
    pub fn new(conn: &'static DatabaseConnection) -> ArticleApplication {
        ArticleApplication {
            repo: ArticleRepositoryImpl { conn },
        }
    }

    pub async fn page(&self, q: PageQuery) -> Result<PageResult<ArticleSummary>, CodedErr> {
        self.repo
            .find_page(q)
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, CodedErr> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }

    pub async fn add(&self, e: Article) -> Result<Article, CodedErr> {
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

    pub async fn update(&self, e: Article) -> Result<Option<Article>, CodedErr> {
        self.repo
            .update(e)
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }
}
