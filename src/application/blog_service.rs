use crate::adapter::api::blog_model::ArticleSummary;
use crate::domain::blog::aggregate::{Article, ArticleRepository};
use crate::infrastructure::model::err::CodedErr;
use crate::infrastructure::model::page::{PageQuery, PageResult};
use uuid::Uuid;

pub struct ArticleApplication<T> {
    pub repo: T,
}

impl<T> ArticleApplication<T>
where
    T: ArticleRepository,
{
    /// 新建
    pub fn new(repo: T) -> ArticleApplication<T> {
        ArticleApplication { repo }
    }

    pub async fn page(&self, q: PageQuery) -> Result<PageResult<ArticleSummary>, CodedErr> {
        let page = self.repo.find_page(q).await;
        page.map_err(|_err| CodedErr::new("B0000".to_string(), _err.to_string()))
    }

    pub async fn find_one(&self, id: Uuid) -> Result<Option<Article>, CodedErr> {
        let result = self.repo.find_one(id).await;
        result.map_err(|_err| CodedErr::new("B0000".to_string(), _err.to_string()))
    }

    pub async fn add(&self, e: Article) -> Result<bool, CodedErr> {
        let result = self.repo.add(e).await;
        result.map_err(|_err| CodedErr::new("B0000".to_string(), _err.to_string()))
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, CodedErr> {
        let result = self.repo.delete(id).await;
        result.map_err(|_err| CodedErr::new("B0000".to_string(), _err.to_string()))
    }

    pub async fn update(&self, e: Article) -> Result<bool, CodedErr> {
        let result = self.repo.update(e).await;
        result.map_err(|_err| CodedErr::new("B0000".to_string(), _err.to_string()))
    }
}
