use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::cache::Cache;
use starriver_shared_base::db::{Connection, Transaction};
use starriver_shared_base::{authentication::PrincipalClaims, db::Revision};
use tracing::{error, info};
use uuid::Uuid;

use crate::dto::category_dto::res::CategoryDetailDto;
use crate::error::CtxError;
use crate::port::category_cache::CACHE_KEY_CATEGORY_LIST;
use crate::port::category_query::CategoryQuery;
use crate::port::category_repository::CategoryRepository;

pub struct CategoryInteractor<Conn, Q, R, C> {
    conn: Conn,
    query: Q,
    repo: R,
    cache: C,
}

impl<Conn, Q, R, C> CategoryInteractor<Conn, Q, R, C>
where
    Conn: Connection,
    Q: CategoryQuery<Conn>,
    R: CategoryRepository<Conn> + CategoryRepository<<Conn as Connection>::Transaction>,
    C: Cache<(), Vec<CategoryDetailDto>>,
{
    pub fn new(conn: Conn, query: Q, repo: R, cache: C) -> Self {
        Self {
            conn,
            query,
            repo,
            cache,
        }
    }

    pub async fn list_all(&self) -> Result<Vec<CategoryDetailDto>, CtxError> {
        self.cache
            .try_get_with(CACHE_KEY_CATEGORY_LIST, async {
                self.query.list_all(&self.conn).await
            })
            .await
            .map_err(|e| {
                error!(error=%e, "database error");
                CtxError::Internal
            })
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
        self.repo
            .insert(&self.conn, category)
            .await
            .map(Ok)
            .inspect(|_| {
                // 插入成功后，缓存需要失效
                self.cache.invalidate_all();
            })?
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
                // 提交成功后，缓存需要失效
                self.cache.invalidate_all();
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
        // 删除成功后，缓存需要失效
        self.cache.invalidate_all();
        Ok(())
    }
}
