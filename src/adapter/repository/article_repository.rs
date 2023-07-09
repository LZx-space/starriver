use chrono::Local;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, PaginatorTrait,
};
use uuid::Uuid;

use crate::adapter::repository::article_po::{ActiveModel, Entity};
use crate::adapter::repository::tag_po::Entity as RTag;
use crate::domain::article::{Article, ArticleRepository};
use crate::domain::tag::Tag;
use crate::infrastructure::model::article_page_item::ArticleSummary;
use crate::infrastructure::model::page::{PageQuery, PageResult};

pub struct ArticleRepositoryImpl<'a> {
    pub conn: &'a DatabaseConnection,
}

#[async_trait]
impl<'a> ArticleRepository for ArticleRepositoryImpl<'a> {
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<ArticleSummary>, DbErr> {
        let paginate = Entity::find()
            .find_also_related(RTag)
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

    async fn find_one(&self, id: Uuid) -> Result<Option<Article>, DbErr> {
        let x = Entity::find_by_id(id).one(self.conn).await?.unwrap();
        let tag: Vec<Tag> = x
            .find_related(RTag)
            .all(self.conn)
            .await?
            .iter()
            .map(|e| Tag {
                id: e.id,
                name: e.name.clone(),
            })
            .collect();
        println!("---------->{:?}", tag.len());
        Ok(Some(Article {
            id,
            title: x.title,
            body: x.body,
            tags: tag,
            author_id: x.author_id,
            create_at: x.create_at,
            modified_records: vec![],
        }))
    }

    async fn add(&self, e: Article) -> Result<bool, DbErr> {
        let model = ActiveModel {
            id: Set(e.id),
            title: Set(e.title),
            body: Set(e.body),
            author_id: Set(e.author_id),
            create_at: Set(Local::now()),
        };
        model.insert(self.conn).await?;
        Ok(true)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(self.conn).await?;
        Ok(result.rows_affected > 0)
    }

    async fn update(&self, e: Article) -> Result<bool, DbErr> {
        let exist_one = Entity::find_by_id(e.id)
            .one(self.conn)
            .await?
            .expect("记录不存在");
        let model = ActiveModel {
            id: Set(exist_one.id),
            title: Set(e.title),
            body: Set(e.body),
            author_id: Set(exist_one.author_id),
            create_at: Set(exist_one.create_at),
        };
        model.update(self.conn).await?;
        Ok(true)
    }
}
