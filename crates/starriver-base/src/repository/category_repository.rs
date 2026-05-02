use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    EntityTrait,
};
use starriver_domain::{
    category::{entity::Category, repository::CategoryRepository},
    common_error::RepositoryError,
    common_model::Revision,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    db::category_do::{ActiveModel, Entity},
    error_mapping::map_db_error,
    util::db::TransactionalConn,
};

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
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, RepositoryError> {
        let category = Entity::find_by_id(id)
            .one(&self.conn)
            .await
            .map_err(map_db_error)?
            .map(|e| Category::from_repo(e.id, e.name));
        Ok(category)
    }

    async fn insert(&self, category: Category) -> Result<Category, RepositoryError> {
        let (id, name) = category.dissolve();
        ActiveModel {
            id: Set(id),
            name: Set(name),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: NotSet,
        }
        .insert(&self.conn)
        .await
        .map(|e| Category::from_repo(e.id, e.name))
        .map_err(map_db_error)
    }

    async fn update(&self, category: Revision<Category>) -> Result<Category, RepositoryError> {
        let (id, name) = category.dissolve().1.dissolve();
        ActiveModel {
            id: Unchanged(id),
            name: Set(name),
            created_at: NotSet,
            updated_at: Set(Some(OffsetDateTime::now_utc())),
        }
        .update(&self.conn)
        .await
        .map(|e| Category::from_repo(e.id, e.name))
        .map_err(map_db_error)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = Entity::delete_by_id(id)
            .exec(&self.conn)
            .await
            .map_err(map_db_error)?
            .rows_affected
            > 0;
        Ok(result)
    }
}
