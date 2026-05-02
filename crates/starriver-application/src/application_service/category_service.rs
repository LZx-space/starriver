use starriver_base::{
    dto::category_dto::res::CategoryDetail, error::ApiError,
    query::category_query_service::CategoryQueryService,
    security::authentication::_default_impl::AuthenticatedUser,
};
use starriver_domain::{
    category::{entity::Category, repository::CategoryRepository},
    common_model::Revision,
};
use tracing::info;
use uuid::Uuid;

pub struct CategoryApplication<Q, R> {
    query: Q,
    repo: R,
}

impl<Q, R> CategoryApplication<Q, R>
where
    Q: CategoryQueryService,
    R: CategoryRepository,
{
    pub fn new(query: Q, repo: R) -> Self {
        Self { query, repo }
    }

    pub async fn list(&self) -> Result<Vec<CategoryDetail>, ApiError> {
        self.query.list().await
    }

    pub async fn find(&self, id: Uuid) -> Result<CategoryDetail, ApiError> {
        self.query
            .find(id)
            .await?
            .ok_or_else(|| ApiError::with_bad_request(format!("category[{}]not exist", id)))
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
        let category = Category::new(name).map_err(ApiError::with_bad_request)?;
        self.repo
            .insert(category)
            .await
            .map_err(ApiError::with_bad_request)
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
        let category = self
            .repo
            .find_by_id(id)
            .await
            .map_err(ApiError::with_bad_request)?;
        let Some(mut category) = category else {
            return Err(ApiError::with_bad_request(""));
        };
        let original = category.clone();
        category.update(name).map_err(ApiError::with_bad_request)?;
        self.repo
            .update(Revision::new(original, category))
            .await
            .map_err(ApiError::with_bad_request)
    }

    pub async fn delete(&self, operator: AuthenticatedUser, id: Uuid) -> Result<(), ApiError> {
        info!(
            user_id = %operator.id,
            category_id = %id,
            "deleting category"
        );
        self.repo
            .delete(id)
            .await
            .map_err(ApiError::with_bad_request)
            .map(|_| ())
    }
}
