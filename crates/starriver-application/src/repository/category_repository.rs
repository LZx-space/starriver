use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    EntityTrait,
};
use starriver_domain::category::{entity::Category, repository::CategoryRepository};
use starriver_infrastructure::{
    error::ApiError, model::aggregate_revision::Revision, util::db::TransactionalConn,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::category_do::{ActiveModel, Entity};

pub struct DefaultCategoryRepository<T> {
    conn: T,
}

impl<T> DefaultCategoryRepository<T>
where
    T: TransactionalConn,
{
    pub fn new(conn: T) -> DefaultCategoryRepository<T> {
        Self { conn }
    }

    /// 字段很少不用开启事务，暂且留着
    #[allow(unused)]
    pub fn conn(self) -> T {
        self.conn
    }
}

impl<T> CategoryRepository for DefaultCategoryRepository<T>
where
    T: TransactionalConn,
{
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, ApiError> {
        let category = Entity::find_by_id(id)
            .one(&self.conn)
            .await?
            .map(|e| Category::from_repo(e.id, e.name, e.create_at, e.update_at));
        Ok(category)
    }

    async fn list(&self) -> Result<Vec<Category>, ApiError> {
        let categories = Entity::find()
            .all(&self.conn)
            .await?
            .into_iter()
            .map(|e| Category::from_repo(e.id, e.name, e.create_at, e.update_at))
            .collect();
        Ok(categories)
    }

    async fn insert(&self, category: Category) -> Result<Category, ApiError> {
        let (id, name, _, _) = category.dissolve();
        ActiveModel {
            id: Set(id),
            name: Set(name),
            create_at: Set(OffsetDateTime::now_utc()),
            update_at: NotSet,
        }
        .insert(&self.conn)
        .await
        .map(|e| Category::from_repo(e.id, e.name, e.create_at, e.update_at))
        .map_err(ApiError::from)
    }

    async fn update(&self, category: Revision<Category>) -> Result<Category, ApiError> {
        let (id, name, _, _) = category.dissolve().1.dissolve();
        ActiveModel {
            id: Unchanged(id),
            name: Set(name),
            create_at: NotSet,
            update_at: Set(Some(OffsetDateTime::now_utc())),
        }
        .update(&self.conn)
        .await
        .map(|e| Category::from_repo(e.id, e.name, e.create_at, e.update_at))
        .map_err(ApiError::from)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, ApiError> {
        let result = Entity::delete_by_id(id)
            .exec(&self.conn)
            .await?
            .rows_affected
            > 0;
        Ok(result)
    }
}
