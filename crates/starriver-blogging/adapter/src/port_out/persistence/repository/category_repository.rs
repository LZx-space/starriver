use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ConnectionTrait, EntityTrait,
};
use starriver_blogging_application::port::category_repository::CategoryRepository;
use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::{db::Revision, error::RepositoryError};
use starriver_shared_framework::{
    db::{DefaultConnection, DefaultTransaction},
    error_mapping::db_2_repo_error,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::category_po::{ActiveModel, Entity};

pub struct DefaultCategoryRepository;

impl DefaultCategoryRepository {
    async fn find_by_id(
        &self,
        conn: &impl ConnectionTrait,
        id: Uuid,
    ) -> Result<Option<Category>, RepositoryError> {
        Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(db_2_repo_error)
            .map(|e| e.map(|e| Category::from_repo(e.id, e.name)))
    }

    async fn insert(
        &self,
        conn: &impl ConnectionTrait,
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

    async fn update(
        &self,
        conn: &impl ConnectionTrait,
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

    async fn delete(&self, conn: &impl ConnectionTrait, id: Uuid) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(id)
            .exec(conn)
            .await
            .map_err(db_2_repo_error)
            .map(|r| r.rows_affected > 0)
    }
}

impl CategoryRepository<DefaultConnection> for DefaultCategoryRepository {
    async fn find_by_id(
        &self,
        conn: &DefaultConnection,
        id: Uuid,
    ) -> Result<Option<Category>, RepositoryError> {
        self.find_by_id(conn, id).await
    }

    async fn insert(
        &self,
        conn: &DefaultConnection,
        category: Category,
    ) -> Result<Category, RepositoryError> {
        self.insert(conn, category).await
    }

    async fn update(
        &self,
        conn: &DefaultConnection,
        category: Revision<Category>,
    ) -> Result<Category, RepositoryError> {
        self.update(conn, category).await
    }

    async fn delete(&self, conn: &DefaultConnection, id: Uuid) -> Result<bool, RepositoryError> {
        self.delete(conn, id).await
    }
}

impl CategoryRepository<DefaultTransaction> for DefaultCategoryRepository {
    async fn find_by_id(
        &self,
        conn: &DefaultTransaction,
        id: Uuid,
    ) -> Result<Option<Category>, RepositoryError> {
        self.find_by_id(conn, id).await
    }

    async fn insert(
        &self,
        conn: &DefaultTransaction,
        category: Category,
    ) -> Result<Category, RepositoryError> {
        self.insert(conn, category).await
    }

    async fn update(
        &self,
        conn: &DefaultTransaction,
        category: Revision<Category>,
    ) -> Result<Category, RepositoryError> {
        self.update(conn, category).await
    }

    async fn delete(&self, conn: &DefaultTransaction, id: Uuid) -> Result<bool, RepositoryError> {
        self.delete(conn, id).await
    }
}
