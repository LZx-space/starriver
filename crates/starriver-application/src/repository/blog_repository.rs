use crate::db::blog_do::ActiveModel;
use crate::db::blog_do::Entity;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use starriver_domain::blog::entity::Blog;
use starriver_domain::blog::repository::BlogRepository;
use starriver_infrastructure::error::error::ApiError;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct DefaultBlogRepository {
    pub conn: &'static DatabaseConnection,
}

impl BlogRepository for DefaultBlogRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, ApiError> {
        Entity::find_by_id(id)
            .one(self.conn)
            .await
            .map(|op| {
                op.map(|e| Blog {
                    id,
                    title: e.title.clone(),
                    body: e.body.clone(),
                    state: e.state.into(),
                    author_id: e.author_id,
                    create_at: e.create_at,
                    update_at: e.update_at,
                })
            })
            .map_err(ApiError::from)
    }

    async fn add(&self, e: Blog) -> Result<Blog, ApiError> {
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
        .map(|e| Blog {
            id: e.id,
            title: e.title,
            body: e.body,
            state: e.state.into(),
            author_id: e.author_id,
            create_at: e.create_at,
            update_at: e.update_at,
        })
        .map_err(ApiError::from)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, ApiError> {
        let result = Entity::delete_by_id(id)
            .exec(self.conn)
            .await?
            .rows_affected
            > 0;
        Ok(result)
    }

    async fn update(&self, e: Blog) -> Result<Option<Blog>, ApiError> {
        let exist = Entity::find_by_id(e.id).one(self.conn).await?;
        match exist {
            None => Ok(None),
            Some(found) => {
                let mut found: ActiveModel = found.into();
                found.title = Set(e.title);
                found.body = Set(e.body);
                found
                    .update(self.conn)
                    .await
                    .map(|updated| {
                        Some(Blog {
                            id: updated.id,
                            title: updated.title,
                            body: updated.body,
                            state: updated.state.into(),
                            author_id: updated.author_id,
                            create_at: updated.create_at,
                            update_at: updated.update_at,
                        })
                    })
                    .map_err(ApiError::from)
            }
        }
    }
}
