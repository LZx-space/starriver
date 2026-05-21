use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    DatabaseConnection, EntityTrait,
};
use starriver_blogging_domain::category::{entity::Category, repository::CategoryRepository};
use starriver_shared_base::{error::RepositoryError, repository::Revision};
use starriver_shared_framework::error_mapping::db_error_2_repo_error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::po::category_po::{ActiveModel, Entity};

pub struct DefaultCategoryRepository {
    conn: DatabaseConnection,
}

impl DefaultCategoryRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl CategoryRepository for DefaultCategoryRepository {
    async fn list_all(&self) -> Result<Vec<Category>, RepositoryError> {
        Entity::find()
            .all(&self.conn)
            .await
            .map(|v| v.into_iter().map(to_entity()).collect())
            .map_err(db_error_2_repo_error)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, RepositoryError> {
        Entity::find_by_id(id)
            .one(&self.conn)
            .await
            .map(|e| e.map(|e| Category::from_repo(e.id, e.name)))
            .map_err(db_error_2_repo_error)
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
        .map_err(db_error_2_repo_error)
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
        .map_err(db_error_2_repo_error)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(id)
            .exec(&self.conn)
            .await
            .map(|r| r.rows_affected > 0)
            .map_err(db_error_2_repo_error)
    }
}

////////////////////////////////////////////////////////////////////////////////////

fn to_entity() -> impl FnMut(super::po::category_po::Model) -> Category {
    |e| Category::from_repo(e.id, e.name)
}
