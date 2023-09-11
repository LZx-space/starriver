use chrono::Local;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QuerySelect,
};
use uuid::Uuid;

use crate::adapter::api::blog_model::ArticleSummary;
use crate::adapter::repository::po::article::Column::Id as ArticleId;
use crate::domain::blog::aggregate::{Article, ArticleRepository};
use crate::domain::blog::value_object::Tag;
use crate::infrastructure::model::page::{PageQuery, PageResult};

pub use super::po::article::ActiveModel as ArticleModel;
pub use super::po::article::Entity as ArticlePo;
pub use super::po::tag::Entity as TagPo;

pub struct ArticleRepositoryImpl {
    pub conn: &'static DatabaseConnection,
}

#[async_trait]
impl ArticleRepository for ArticleRepositoryImpl {
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<ArticleSummary>, DbErr> {
        let articles = ArticlePo::find()
            .find_with_related(TagPo)
            .offset(q.page * q.page_size)
            .limit(q.page_size)
            .all(self.conn)
            .await?
            .iter()
            .map(|e| {
                let article = &e.0;
                let tag: Vec<String> = e.1.iter().map(|t| t.name.to_string()).collect();
                ArticleSummary {
                    id: article.id,
                    title: article.title.clone(),
                    release_date: article.create_at.to_string(),
                    tags: tag,
                }
            })
            .collect();
        let record_total = ArticlePo::find()
            .select_only()
            .column(ArticleId)
            .count(self.conn)
            .await?;
        Ok(PageResult {
            page: q.page,
            page_size: q.page_size,
            record_total,
            records: articles,
        })
    }

    async fn find_one(&self, id: Uuid) -> Result<Option<Article>, DbErr> {
        let option = ArticlePo::find_by_id(id)
            .find_with_related(TagPo)
            .all(self.conn)
            .await?;
        if option.is_empty() {
            Ok(None)
        } else {
            Ok(option.first().map(|e| {
                let article_model = &e.0;
                let tags: Vec<Tag> =
                    e.1.iter()
                        .map(|t| Tag {
                            id: t.id,
                            name: t.name.to_string(),
                        })
                        .collect();
                Article {
                    id,
                    title: article_model.title.clone(),
                    body: article_model.body.clone(),
                    tags,
                    author_id: article_model.author_id.clone(),
                    create_at: article_model.create_at.clone(),
                    modified_records: vec![],
                }
            }))
        }
    }

    async fn add(&self, e: Article) -> Result<bool, DbErr> {
        let model = ArticleModel {
            id: Set(e.id),
            title: Set(e.title),
            body: Set(e.body),
            author_id: Set(e.author_id),
            create_at: Set(Local::now()),
            update_at: Set(None),
        };
        model.insert(self.conn).await?;
        Ok(true)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        let result = ArticlePo::delete_by_id(id).exec(self.conn).await?;
        Ok(result.rows_affected > 0)
    }

    async fn update(&self, e: Article) -> Result<bool, DbErr> {
        let exist_one = ArticlePo::find_by_id(e.id)
            .one(self.conn)
            .await?
            .map(|p| ArticleModel {
                id: Set(p.id),
                title: Set(p.title),
                body: Set(p.body),
                author_id: Set(p.author_id),
                create_at: Set(p.create_at),
                update_at: Set(p.update_at),
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
