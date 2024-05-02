use anyhow::Error;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect};
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
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<ArticleSummary>, Error> {
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
        Ok(PageResult::new(q.page, q.page_size, record_total, articles))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Error> {
        ArticlePo::find_by_id(id)
            .one(self.conn)
            .await
            .map(|op| {
                op.map(|e| Article {
                    id,
                    title: e.title.clone(),
                    body: e.body.clone(),
                    state: e.state.into(),
                    author_id: e.author_id,
                    create_at: e.create_at,
                    update_at: e.update_at,
                })
            })
            .map_err(|e| Error::from(e))
    }

    async fn add(&self, e: Article) -> Result<Article, Error> {
        ArticleModel {
            id: Set(e.id),
            title: Set(e.title),
            body: Set(e.body),
            state: Set(Default::default()),
            author_id: Set(e.author_id),
            create_at: Set(Local::now()),
            update_at: Set(None),
        }
        .insert(self.conn)
        .await
        .map(|e| Article {
            id: e.id,
            title: e.title,
            body: e.body,
            state: e.state.into(),
            author_id: e.author_id,
            create_at: e.create_at,
            update_at: e.update_at,
        })
        .map_err(|e| Error::from(e))
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, Error> {
        ArticlePo::delete_by_id(id)
            .exec(self.conn)
            .await
            .map(|e| e.rows_affected > 0)
            .map_err(|e| Error::from(e))
    }

    async fn update(&self, e: Article) -> Result<Option<Article>, Error> {
        let result = ArticlePo::find_by_id(e.id).one(self.conn).await;
        return match result {
            Ok(op) => match op {
                None => Ok(None),
                Some(mut model) => {
                    let active_model = ArticleModel {
                        id: Set(model.id),
                        title: Set(e.title),
                        body: Set(e.body),
                        state: Default::default(),
                        author_id: Set(model.author_id),
                        create_at: Set(model.create_at),
                        update_at: Set(Some(Local::now())),
                    };
                    active_model
                        .update(self.conn)
                        .await
                        .map(|e| {
                            Some(Article {
                                id: e.id,
                                title: e.title,
                                body: e.body,
                                state: e.state.into(),
                                author_id: e.author_id,
                                create_at: e.create_at,
                                update_at: e.update_at,
                            })
                        })
                        .map_err(|e| Error::from(e))
                }
            },
            Err(err) => Err(Error::from(err)),
        };
    }
}
