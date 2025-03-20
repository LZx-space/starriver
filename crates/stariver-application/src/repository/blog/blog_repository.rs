use super::po::article::ActiveModel;
use super::po::article::Column;
use super::po::article::Entity;
use anyhow::Error;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect};
use stariver_domain::blog::aggregate::Article;
use stariver_domain::blog::repository::ArticleRepository;
use stariver_infrastructure::model::blog::ArticleSummary;
use stariver_infrastructure::model::page::{PageQuery, PageResult};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct ArticleRepositoryImpl {
    pub conn: &'static DatabaseConnection,
}

impl ArticleRepository for ArticleRepositoryImpl {
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<ArticleSummary>, Error> {
        let articles = Entity::find()
            .select_only()
            .columns([Column::Id, Column::Title, Column::CreateAt])
            .offset(q.page * q.page_size)
            .limit(q.page_size)
            .into_model::<ArticleSummary>()
            .all(self.conn)
            .await?;
        let record_total = Entity::find()
            .select_only()
            .column(Column::Id)
            .count(self.conn)
            .await?;
        Ok(PageResult::new(q.page, q.page_size, record_total, articles))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Error> {
        Entity::find_by_id(id)
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
            .map_err(Error::from)
    }

    async fn add(&self, e: Article) -> Result<Article, Error> {
        ActiveModel {
            id: Set(e.id),
            title: Set(e.title),
            body: Set(e.body),
            state: Set(Default::default()),
            author_id: Set(e.author_id),
            create_at: Set(OffsetDateTime::now_utc()),
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
        .map_err(Error::from)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, Error> {
        Entity::delete_by_id(id)
            .exec(self.conn)
            .await
            .map(|e| e.rows_affected > 0)
            .map_err(Error::from)
    }

    async fn update(&self, e: Article) -> Result<Option<Article>, Error> {
        let exist = Entity::find_by_id(e.id).one(self.conn).await;
        match exist {
            Ok(op) => match op {
                None => Ok(None),
                Some(found) => {
                    let mut found: ActiveModel = found.into();
                    found.title = Set(e.title);
                    found.body = Set(e.body);
                    found
                        .update(self.conn)
                        .await
                        .map(|updated| {
                            Some(Article {
                                id: updated.id,
                                title: updated.title,
                                body: updated.body,
                                state: updated.state.into(),
                                author_id: updated.author_id,
                                create_at: updated.create_at,
                                update_at: updated.update_at,
                            })
                        })
                        .map_err(Error::from)
                }
            },
            Err(err) => Err(Error::from(err)),
        }
    }
}
