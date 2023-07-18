use chrono::Local;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};

use crate::adapter::api::blog_model::ArticleSummary;
use crate::domain::blog::article::{Article, ArticleRepository};
use crate::infrastructure::model::page::{PageQuery, PageResult};

pub use super::po::article::ActiveModel as ArticleModel;
pub use super::po::article::Entity as ArticlePo;
pub use super::po::tag::Entity as TagPo;

pub struct ArticleRepositoryImpl<'a> {
    pub conn: &'a DatabaseConnection,
}

#[async_trait]
impl<'a> ArticleRepository for ArticleRepositoryImpl<'a> {
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<ArticleSummary>, DbErr> {
        let paginate = ArticlePo::find()
            .find_also_related(TagPo)
            .paginate(self.conn, q.page_size);
        let articles = paginate
            .fetch()
            .await?
            .iter()
            .map(|e| {
                let article = &e.0;
                let tag = &e.1;
                println!("{:?}", tag);
                ArticleSummary {
                    id: article.id,
                    title: article.title.clone(),
                    release_date: article.create_at.to_string(),
                    tags: vec![],
                }
            })
            .collect();
        let record_total = paginate.num_items().await?;
        Ok(PageResult {
            page: q.page as u8,
            page_size: q.page_size as u8,
            record_total: record_total as u8,
            records: articles,
        })
    }

    async fn find_one(&self, id: i64) -> Result<Option<Article>, DbErr> {
        // Ok(Some(Article {
        //     id,
        //     title: po.title,
        //     body: po.body,
        //     tags: vec![],
        //     author_id: po.author_id,
        //     create_at: po.create_at,
        //     modified_records: vec![],
        // }))
        todo!()
    }

    async fn add(&self, e: Article) -> Result<bool, DbErr> {
        let model = ArticleModel {
            id: Set(e.id),
            title: Set(e.title),
            body: Set(e.body),
            author_id: Set(e.author_id),
            create_at: Set(Local::now()),
        };
        model.insert(self.conn).await?;
        Ok(true)
    }

    async fn delete(&self, id: i64) -> Result<bool, DbErr> {
        let result = ArticlePo::delete_by_id(id).exec(self.conn).await?;
        Ok(result.rows_affected > 0)
    }

    async fn update(&self, e: Article) -> Result<bool, DbErr> {
        //     let exist_one = ArticlePo::find_by_id(e.id)
        //         .one(self.conn)
        //         .await?
        //         .map_or_else(Some(false), |p| ArticleModel {
        //             id: Set(p.id),
        //             title: Set(p.title),
        //             body: Set(p.body),
        //             author_id: Set(p.author_id),
        //             create_at: Set(p.create_at),
        //         });
        //     model.update(self.conn).await?;
        //     Ok(true)
        todo!()
    }
}
