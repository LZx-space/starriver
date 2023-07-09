use uuid::Uuid;

use crate::domain::article::{Article, ArticleRepository};
use crate::infrastructure::model::article_page_item::ArticleSummary;
use crate::infrastructure::model::err::BizErr;
use crate::infrastructure::model::page::{PageQuery, PageResult};

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

    pub async fn page(&self, q: PageQuery) -> Result<PageResult<ArticleSummary>, BizErr> {
        let page = self.repo.find_page(q).await;
        page.map_err(|_err| BizErr::new(_err.to_string()))
    }

    pub async fn find_one(&self, id: Uuid) -> Result<Option<Article>, BizErr> {
        let result = self.repo.find_one(id).await;
        result.map_err(|_err| BizErr::new(_err.to_string()))
    }

    pub async fn add(&self, e: Article) -> Result<bool, BizErr> {
        let result = self.repo.add(e).await;
        result.map_err(|_err| BizErr::new(_err.to_string()))
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, BizErr> {
        let result = self.repo.delete(id).await;
        result.map_err(|_err| BizErr::new(_err.to_string()))
    }

    pub async fn update(&self, e: Article) -> Result<bool, BizErr> {
        let result = self.repo.update(e).await;
        result.map_err(|_err| BizErr::new(_err.to_string()))
    }
}
