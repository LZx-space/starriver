use sea_orm::DatabaseConnection;
use starriver_domain::category::{entity::Category, repository::CategoryRepository};
use starriver_infrastructure::{
    error::ApiError, model::aggregate_revision::Revision,
    security::authentication::_default_impl::AuthenticatedUser,
};
use tracing::info;
use uuid::Uuid;

use crate::repository::category_repository::DefaultCategoryRepository;

pub struct CategoryApplication {
    repo: DefaultCategoryRepository<DatabaseConnection>,
}

impl CategoryApplication {
    pub fn new(conn: DatabaseConnection) -> Self {
        let repo = DefaultCategoryRepository::new(conn.clone());
        Self { repo }
    }

    pub async fn list(&self) -> Result<Vec<Category>, ApiError> {
        self.repo.list().await
    }

    pub async fn create(
        &self,
        operator: AuthenticatedUser,
        name: String,
    ) -> Result<Category, ApiError> {
        info!(
            user_id = %operator.id,
            category_name = %name,
            "creating category"
        );
        let category = Category::new(name)?;
        self.repo.insert(category).await
    }

    pub async fn update(
        &self,
        operator: AuthenticatedUser,
        id: Uuid,
        name: String,
    ) -> Result<Category, ApiError> {
        info!(
            user_id = %operator.id,
            category_id = %id,
            "updating category"
        );
        let category = self.repo.find_by_id(id).await?;
        let mut category = match category {
            Some(category) => category,
            None => return Err(ApiError::with_bad_request("")),
        };
        let original = category.clone();
        category.update(name)?;
        self.repo.update(Revision::new(original, category)).await
    }

    pub async fn delete(&self, operator: AuthenticatedUser, id: Uuid) -> Result<(), ApiError> {
        info!(
            user_id = %operator.id,
            category_id = %id,
            "deleting category"
        );
        self.repo.delete(id).await.map(|_| ())
    }
}
