use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QuerySelect,
};
use uuid::Uuid;

use crate::domain::blog::aggregate::Article;
use crate::domain::blog::repository::ArticleRepository;
use crate::infrastructure::model::blog::ArticleSummary;
use crate::infrastructure::model::page::{PageQuery, PageResult};

pub use super::po::article::ActiveModel as ArticleModel;
pub use super::po::article::Column;
pub use super::po::article::Entity as ArticlePo;

pub struct ArticleRepositoryImpl {
    pub conn: &'static DatabaseConnection,
}

impl ArticleRepository for ArticleRepositoryImpl {
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<ArticleSummary>, DbErr> {
        let articles = ArticlePo::find()
            .select_only()
            .columns([Column::Id, Column::Title, Column::CreateAt])
            .offset(q.page * q.page_size)
            .limit(q.page_size)
            .into_model::<ArticleSummary>()
            .all(self.conn)
            .await?;
        let record_total = ArticlePo::find()
            .select_only()
            .column(Column::Id)
            .count(self.conn)
            .await?;
        Ok(PageResult {
            page: q.page,
            page_size: q.page_size,
            record_total,
            records: articles,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, DbErr> {
        let article = ArticlePo::find_by_id(id)
            .one(self.conn)
            .await?
            .map(|e| Article {
                id,
                title: e.title.clone(),
                body: e.body.clone(),
                state: e.state.into(),
                author_id: e.author_id.clone(),
                create_at: e.create_at.clone(),
                modified_records: vec![],
            });
        Ok(article)
    }

    async fn add(&self, e: Article) -> Result<bool, DbErr> {
        let model = ArticleModel {
            id: Set(e.id),
            title: Set(e.title),
            body: Set(e.body),
            state: Set(Default::default()),
            author_id: Set(e.author_id),
            create_at: Set(Local::now()),
            update_at: Set(None),
        };
        model.insert(self.conn).await?;
        Ok(true)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DbErr> {
        let result = ArticlePo::delete_by_id(id).exec(self.conn).await?;
        Ok(result.rows_affected > 0)
    }

    async fn update(&self, e: Article) -> Result<bool, DbErr> {
        let exist_one = ArticlePo::find_by_id(e.id)
            .one(self.conn)
            .await?
            .map(|model| ArticleModel {
                id: Set(model.id),
                title: Set(model.title),
                body: Set(model.body),
                state: Default::default(),
                author_id: Set(model.author_id),
                create_at: Set(model.create_at),
                update_at: Set(Some(Local::now())),
            });
        match exist_one {
            None => Ok(false),
            Some(model) => {
                model.update(self.conn).await?;
                Ok(true)
            }
        }
    }
}
