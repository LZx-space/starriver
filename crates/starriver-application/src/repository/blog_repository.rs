use crate::db::blog_do::ActiveModel;
use crate::db::blog_do::Entity;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveValue::Unchanged;
use sea_orm::{ActiveModelTrait, EntityTrait};
use starriver_domain::blog::entity::Blog;
use starriver_domain::blog::repository::BlogRepository;
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::error::Cause;
use starriver_infrastructure::util::db::TransactionalConn;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct DefaultBlogRepository<T> {
    conn: T,
}

impl<T> DefaultBlogRepository<T>
where
    T: TransactionalConn,
{
    pub fn new(conn: T) -> DefaultBlogRepository<T> {
        Self { conn }
    }

    pub fn conn(self) -> T {
        self.conn
    }
}

impl<T> BlogRepository for DefaultBlogRepository<T>
where
    T: TransactionalConn,
{
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, ApiError> {
        find_by_id(&self.conn, id).await
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
        .insert(&self.conn)
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
            .exec(&self.conn)
            .await?
            .rows_affected
            > 0;
        Ok(result)
    }

    async fn update(&self, blog: Blog) -> Result<Blog, ApiError> {
        match find_by_id(&self.conn, blog.id).await? {
            Some(found) => {
                let mut title = Unchanged(found.title);
                title.set_if_not_equals(blog.title);

                let mut body = Unchanged(found.body);
                body.set_if_not_equals(blog.body);

                let mut state = Unchanged(found.state.into());
                state.set_if_not_equals(blog.state.into());

                let mut author_id = Unchanged(found.author_id);
                author_id.set_if_not_equals(blog.author_id);

                let model = ActiveModel {
                    id: Unchanged(found.id),
                    title,
                    body,
                    state,
                    author_id,
                    create_at: Unchanged(found.create_at),
                    update_at: Set(Some(OffsetDateTime::now_utc())),
                };

                model
                    .update(&self.conn)
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
