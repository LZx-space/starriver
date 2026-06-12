use sea_orm::{ConnectionTrait, TransactionTrait};
use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::{authentication::PrincipalClaims, repository::Revision};
use tracing::{error, info};
use uuid::Uuid;

use crate::dto::category_dto::res::CategoryDetailDto;
use crate::error::CtxError;
use crate::port::category_query::CategoryQuery;
use crate::port::category_repository::CategoryRepository;

pub struct CategoryApplication<Conn, Q, R> {
    conn: Conn,
    query: Q,
    repo: R,
}

impl<Conn, Q, R> CategoryApplication<Conn, Q, R>
where
    Conn: ConnectionTrait + TransactionTrait,
    Q: CategoryQuery,
    R: CategoryRepository,
{
    pub fn new(conn: Conn, query: Q, repo: R) -> Self {
        Self { conn, query, repo }
    }

    pub async fn list_all(&self) -> Result<Vec<CategoryDetailDto>, CtxError> {
        self.query.list_all(&self.conn).await.map(Ok)?
    }

    pub async fn find(&self, id: Uuid) -> Result<Category, CtxError> {
        self.repo
            .find_by_id(&self.conn, id)
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
        self.repo.insert(&self.conn, category).await.map(Ok)?
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

        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;

        let result = async {
            let category = self.repo.find_by_id(&tx, id).await?;
            let mut category = match category {
                Some(category) => category,
                None => return Err(CtxError::NotFound(format!("category[{}]not exist", id))),
            };
            let original = category.clone();
            category.update(name)?;
            self.repo
                .update(&tx, Revision::new(original, category))
                .await
                .map(Ok)?
        }
        .await;

        match result {
            Ok(val) => {
                tx.commit().await.map_err(|e| {
                    error!(user_id=%operator.sub, error=%e, "commit transaction failed");
                    CtxError::Internal
                })?;
                Ok(val)
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    error!(user_id=%operator.sub, error=%e, "rollback transaction failed");
                    CtxError::Internal
                })?;
                Err(e)
            }
        }
    }

    pub async fn delete(&self, operator: PrincipalClaims, id: Uuid) -> Result<(), CtxError> {
        info!(
            user_id = %operator.sub,
            category_id = %id,
            "deleting category"
        );
        self.repo.delete(&self.conn, id).await?;
        Ok(())
    }
}
