use crate::db::blog_do::ActiveModel;
use crate::db::blog_do::Entity;
use crate::db::blog_do::Model;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use starriver_domain::blog::entity::Blog;
use starriver_domain::blog::repository::BlogRepository;
use starriver_infrastructure::error::error::ApiError;
use starriver_infrastructure::error::error::Cause;
use starriver_infrastructure::update_active_model_on_change;
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

    async fn add(&self, blog: Blog) -> Result<Blog, ApiError> {
        ActiveModel {
            id: Set(blog.id),
            title: Set(blog.title),
            body: Set(blog.body),
            state: Set(Default::default()),
            author_id: Set(blog.author_id),
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

    async fn update(&self, blog: Blog) -> Result<Blog, ApiError> {
        match Entity::find_by_id(blog.id).one(self.conn).await? {
            Some(found) => {
                let mut model: ActiveModel = found.into();

                let any_updated = update_active_model_on_change!(
                    model, blog, title, body, author_id, create_at, update_at
                );
                if !any_updated {
                    return Err(ApiError::new(Cause::ClientBadRequest, "none field updated"));
                }
                return model
                    .update(self.conn)
                    .await
                    .map(fun_name)
                    .map_err(ApiError::from);
            }
            None => {
                return Err(ApiError::new(Cause::ClientBadRequest, "Blog not found"));
            }
        };
    }
}

fn fun_name(updated: Model) -> Blog {
    Blog {
        id: updated.id,
        title: updated.title,
        body: updated.body,
        state: updated.state.into(),
        author_id: updated.author_id,
        create_at: updated.create_at,
        update_at: updated.update_at,
    }
}
