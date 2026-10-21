use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ConnectionTrait, EntityTrait,
};
use starriver_blogging_application::port::category_repository::CategoryRepository;
use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::{
    db::{Executor, Revision},
    error::RepositoryError,
};
use starriver_shared_infra::error_mapping::db_2_repo_error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::category_po::{ActiveModel, Entity};

pub struct DefaultCategoryRepository;

impl<T> CategoryRepository<T> for DefaultCategoryRepository
where
    T: Executor + ConnectionTrait,
{
    async fn find_by_id(&self, conn: &T, id: Uuid) -> Result<Option<Category>, RepositoryError> {
        Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(db_2_repo_error)
            .map(|e| e.map(|e| Category::from_repo(e.id, e.name)))
    }

    async fn exists_by_id(&self, conn: &T, id: Uuid) -> Result<bool, RepositoryError> {
        Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(db_2_repo_error)
            .map(|e| e.is_some())
    }

    async fn insert(&self, conn: &T, category: Category) -> Result<Category, RepositoryError> {
        let (id, name) = category.dissolve();
        ActiveModel {
            id: Set(id),
            name: Set(name),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: NotSet,
        }
        .insert(conn)
        .await
        .map_err(db_2_repo_error)
        .map(|e| Category::from_repo(e.id, e.name))
    }

    async fn update(
        &self,
        conn: &T,
        category: Revision<Category>,
    ) -> Result<Category, RepositoryError> {
        let (id, name) = category.dissolve().1.dissolve();
        ActiveModel {
            id: Unchanged(id),
            name: Set(name),
            created_at: NotSet,
            updated_at: Set(Some(OffsetDateTime::now_utc())),
        }
        .update(conn)
        .await
        .map_err(db_2_repo_error)
        .map(|e| Category::from_repo(e.id, e.name))
    }

    async fn delete(&self, conn: &T, id: Uuid) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(id)
            .exec(conn)
            .await
            .map_err(db_2_repo_error)
            .map(|r| r.rows_affected > 0)
    }
}
