use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ConnectionTrait, EntityTrait,
};
use starriver_blogging_application::port::category_repository::CategoryRepository;
use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::{error::RepositoryError, repository::Revision};
use starriver_shared_framework::error_mapping::db_2_repo_error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::category_po::{ActiveModel, Entity};

pub struct DefaultCategoryRepository;

impl CategoryRepository for DefaultCategoryRepository {
    async fn find_by_id<C: ConnectionTrait>(
        &self,
        conn: &C,
        id: Uuid,
    ) -> Result<Option<Category>, RepositoryError> {
        Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(db_2_repo_error)
            .map(|e| e.map(|e| Category::from_repo(e.id, e.name)))
    }

    async fn insert<C: ConnectionTrait>(
        &self,
        conn: &C,
        category: Category,
    ) -> Result<Category, RepositoryError> {
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

    async fn update<C: ConnectionTrait>(
        &self,
        conn: &C,
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

    async fn delete<C: ConnectionTrait>(
        &self,
        conn: &C,
        id: Uuid,
    ) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(id)
            .exec(conn)
            .await
            .map_err(db_2_repo_error)
            .map(|r| r.rows_affected > 0)
    }
}
