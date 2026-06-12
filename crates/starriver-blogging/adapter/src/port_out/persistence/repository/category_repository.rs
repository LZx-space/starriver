use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    DatabaseConnection, EntityTrait,
};
use starriver_blogging_application::port::category_repository::CategoryRepository;
use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::{error::RepositoryError, repository::Revision};
use starriver_shared_framework::error_mapping::db_2_repo_error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    port_in::state::{CACHE_KEY_CATEGORY_LIST, CatagoryListCache},
    port_out::persistence::po::category_po::{ActiveModel, Entity},
};

pub struct DefaultCategoryRepository {
    conn: DatabaseConnection,
    cache: CatagoryListCache,
}

impl DefaultCategoryRepository {
    pub fn new(conn: DatabaseConnection, cache: CatagoryListCache) -> Self {
        Self { conn, cache }
    }
}

impl CategoryRepository for DefaultCategoryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, RepositoryError> {
        Entity::find_by_id(id)
            .one(&self.conn)
            .await
            .map(|e| e.map(|e| Category::from_repo(e.id, e.name)))
            .map_err(db_2_repo_error)
    }

    async fn insert(&self, category: Category) -> Result<Category, RepositoryError> {
        let (id, name) = category.dissolve();
        let category = ActiveModel {
            id: Set(id),
            name: Set(name),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: NotSet,
        }
        .insert(&self.conn)
        .await
        .map(|e| Category::from_repo(e.id, e.name))
        .map_err(db_2_repo_error)?;

        self.cache.invalidate(&CACHE_KEY_CATEGORY_LIST).await;
        Ok(category)
    }

    async fn update(&self, category: Revision<Category>) -> Result<Category, RepositoryError> {
        let (id, name) = category.dissolve().1.dissolve();
        let category = ActiveModel {
            id: Unchanged(id),
            name: Set(name),
            created_at: NotSet,
            updated_at: Set(Some(OffsetDateTime::now_utc())),
        }
        .update(&self.conn)
        .await
        .map(|e| Category::from_repo(e.id, e.name))
        .map_err(db_2_repo_error)?;

        self.cache.invalidate(&CACHE_KEY_CATEGORY_LIST).await;
        Ok(category)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = Entity::delete_by_id(id)
            .exec(&self.conn)
            .await
            .map(|r| r.rows_affected > 0)
            .map_err(db_2_repo_error)?;

        self.cache.invalidate(&CACHE_KEY_CATEGORY_LIST).await;
        Ok(result)
    }
}
