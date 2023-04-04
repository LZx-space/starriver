use chrono::Local;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use uuid::Uuid;

use crate::adapter::repository::article_po;
use crate::domain::article::{Article, ArticleRepository};
use crate::infrastructure::model::article_page_item::ArticlePageItem;
use crate::infrastructure::model::page::{PageQuery, PageResult};

pub struct ArticleRepositoryImpl<'a> {
    pub conn: &'a DatabaseConnection,
}

#[async_trait]
impl<'a> ArticleRepository for ArticleRepositoryImpl<'a> {
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<ArticlePageItem>, DbErr> {
        let select = article_po::Entity::find()
            .paginate(self.conn, q.page_size)
            .fetch_page(q.page)
            .await?;
        let mut articles = vec![];
        for x in select {
            let article = ArticlePageItem {
                id: x.id,
                title: x.title,
                release_date: "".to_string(),
                tags: vec![],
            };
            articles.push(article);
        }
        Ok(PageResult {
            page: 0,
            page_size: 0,
            record_total: 0,
            records: articles,
        })
    }

    async fn find_one(&self, id: Uuid) -> Result<Option<Article>, DbErr> {
        let x = article_po::Entity::find_by_id(id).one(self.conn).await?;
        Ok(x.map(|x| Article {
            id,
            title: x.title,
            body: x.body,
            tags: vec![],
            creator_id: x.creator_id,
            create_time: x.create_time,
            modified_records: vec![],
        }))
    }

    async fn add(&self, e: Article) -> Result<bool, DbErr> {
        let model = article_po::ActiveModel {
            id: Set(e.id),
            title: Set(e.title),
            body: Set(e.body),
            creator_id: Set(e.creator_id),
            create_time: Set(Local::now()),
            tag_ids: Default::default(),
        };
        model.insert(self.conn).await?;
        Ok(true)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        let result = article_po::Entity::delete_by_id(id).exec(self.conn).await?;
        Ok(result.rows_affected > 0)
    }

    async fn update(&self, e: Article) -> Result<bool, DbErr> {
        let exist_one = article_po::Entity::find_by_id(e.id)
            .one(self.conn)
            .await?
            .expect("记录不存在");
        let model = article_po::ActiveModel {
            id: Set(exist_one.id),
            title: Set(e.title),
            body: Set(e.body),
            creator_id: Set(exist_one.creator_id),
            create_time: Set(exist_one.create_time),
            tag_ids: Set(exist_one.tag_ids),
        };
        model.update(self.conn).await?;
        Ok(true)
    }
}
