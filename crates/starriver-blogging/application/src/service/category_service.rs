use starriver_blogging_domain::category::{entity::Category, repository::CategoryRepository};
use starriver_shared_base::{authentication::PrincipalClaims, repository::Revision};
use tracing::info;
use uuid::Uuid;

use crate::dto::category_dto::res::CategoryDetailDto;
use crate::error::CtxError;
use crate::port_out::category_query_port::CategoryQueryPort;

pub struct CategoryApplication<Q, R> {
    query: Q,
    repo: R,
}

impl<Q, R> CategoryApplication<Q, R>
where
    Q: CategoryQueryPort,
    R: CategoryRepository,
{
    pub fn new(query: Q, repo: R) -> Self {
        Self { query, repo }
    }

    pub async fn list_all(&self) -> Result<Vec<CategoryDetailDto>, CtxError> {
        self.query.list_all().await.map(Ok)?
    }

    pub async fn find(&self, id: Uuid) -> Result<Category, CtxError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| CtxError::NotFound(format!("category[{}]not exist", id)))
    }

    pub async fn create(
        &self,
        operator: PrincipalClaims,
        name: String,
    ) -> Result<Category, CtxError> {
        info!(
            user_id = %operator.sub,
            category_name = %name,
            "creating category"
        );
        let category = Category::new(name)?;
        self.repo.insert(category).await.map(Ok)?
    }

    pub async fn update(
        &self,
        operator: PrincipalClaims,
        id: Uuid,
        name: String,
    ) -> Result<Category, CtxError> {
        info!(
            user_id = %operator.sub,
            category_id = %id,
            "updating category"
        );
        let category = self.repo.find_by_id(id).await?;
        let mut category = match category {
            Some(category) => category,
            None => return Err(CtxError::NotFound(format!("category[{}]not exist", id))),
        };
        let original = category.clone();
        category.update(name)?;
        self.repo
            .update(Revision::new(original, category))
            .await
            .map(Ok)?
    }

    pub async fn delete(&self, operator: PrincipalClaims, id: Uuid) -> Result<(), CtxError> {
        info!(
            user_id = %operator.sub,
            category_id = %id,
            "deleting category"
        );
        self.repo.delete(id).await?;
        Ok(())
    }
}
