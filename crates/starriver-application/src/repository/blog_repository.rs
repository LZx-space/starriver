use crate::db::blog_do::ActiveModel;
use crate::db::blog_do::BlogStateDo;
use crate::db::blog_do::Entity;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use starriver_domain::blog::entity::Blog;
use starriver_domain::blog::repository::BlogRepository;
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::error::Cause;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct DefaultBlogRepository {
    pub conn: &'static DatabaseConnection,
}

impl BlogRepository for DefaultBlogRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, ApiError> {
        find_by_id(self.conn, id).await
    }

    async fn add(&self, blog: Blog) -> Result<Blog, ApiError> {
        ActiveModel {
            id: Set(blog.id),
            title: Set(blog.title),
            body: Set(blog.body),
            state: Set(Default::default()),
            author_id: Set(blog.author_id),
            create_at: Set(OffsetDateTime::now_utc()),
            update_at: NotSet,
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
        self.conn
            .transaction::<_, Blog, ApiError>(|tx| {
                Box::pin(async move {
                    match find_by_id(tx, blog.id).await? {
                        Some(found) => {
                            let mut model = ActiveModel::builder()
                                .set_id(found.id)
                                .set_update_at(Some(OffsetDateTime::now_utc()));
                            if found.title != blog.title {
                                model = model.set_title(blog.title);
                            }
                            if found.body != blog.body {
                                model = model.set_body(blog.body);
                            }
                            if found.state != blog.state {
                                let state: BlogStateDo = blog.state.into();
                                model = model.set_state(state);
                            }
                            model
                                .update(tx)
                                .await
                                .map(|updated| Blog {
                                    id: updated.id,
                                    title: updated.title,
                                    body: updated.body,
                                    state: updated.state.into(),
                                    author_id: updated.author_id,
                                    create_at: updated.create_at,
                                    update_at: updated.update_at,
                                })
                                .map_err(ApiError::from)
                        }
                        None => Err(ApiError::new(Cause::ClientBadRequest, "Blog not found")),
                    }
                })
            })
            .await
            .map_err(ApiError::from)
    }
}

async fn find_by_id(
    conn: &impl sea_orm::ConnectionTrait,
    id: Uuid,
) -> Result<Option<Blog>, ApiError> {
    Entity::find_by_id(id)
        .one(conn)
        .await
        .map(|op| {
            op.map(|e| Blog {
                id,
                title: e.title,
                body: e.body,
                state: e.state.into(),
                author_id: e.author_id,
                create_at: e.create_at,
                update_at: e.update_at,
            })
        })
        .map_err(ApiError::from)
}
