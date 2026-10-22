use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::cache::{Cache, Invalidator};
use starriver_shared_base::db::{Connection, Transaction};
use starriver_shared_base::error::RepositoryError;
use starriver_shared_base::{authentication::principal::DefaultUser, db::Revision};
use tracing::{error, info};
use uuid::Uuid;

use crate::dto::category_dto::res::CategoryDetailDto;
use crate::error::CtxError;
use crate::port::category_cache::{CACHE_KEY_CATEGORY_LIST, CategoryListKey};
use crate::port::category_query::CategoryQuery;
use crate::port::category_repository::CategoryRepository;
use crate::port::post_cache::PostPageKey;

pub struct CategoryInteractor<Conn, Q, R, C, PPI, PII> {
    conn: Conn,
    query: Q,
    repo: R,
    cache: C,
    post_page_cache_invalidator: PPI,
    post_item_cache_invalidator: PII,
}

impl<Conn, Q, R, C, PPI, PII> CategoryInteractor<Conn, Q, R, C, PPI, PII>
where
    Conn: Connection,
    Q: CategoryQuery<Conn>,
    R: CategoryRepository<Conn> + CategoryRepository<<Conn as Connection>::Transaction>,
    C: Cache<CategoryListKey, Vec<CategoryDetailDto>>,
    PPI: Invalidator<PostPageKey>,
    PII: Invalidator<Uuid>,
{
    pub fn new(
        conn: Conn,
        query: Q,
        repo: R,
        cache: C,
        post_page_cache_invalidator: PPI,
        post_item_cache_invalidator: PII,
    ) -> Self {
        Self {
            conn,
            query,
            repo,
            cache,
            post_page_cache_invalidator,
            post_item_cache_invalidator,
        }
    }

    pub async fn list_all(&self) -> Result<Vec<CategoryDetailDto>, CtxError> {
        self.cache
            .try_get_with(CACHE_KEY_CATEGORY_LIST, async {
                self.query.list_all(&self.conn).await
            })
            .await
            .map_err(|e| CtxError::internal("list categories failed", e))
    }

    pub async fn find(&self, id: Uuid) -> Result<Category, CtxError> {
        self.repo
            .find_by_id(&self.conn, id)
            .await?
            .ok_or_else(|| CtxError::NotFound(format!("category[{}]not exist", id)))
    }

    pub async fn create(&self, operator: DefaultUser, name: String) -> Result<Category, CtxError> {
        info!(
            user_id = %operator.id,
            category_name = %name,
            "creating category"
        );
        let category = Category::new(name)?;
        let created = self.repo.insert(&self.conn, category).await?;
        // 插入成功后，缓存需要失效
        self.cache.invalidate_all();
        Ok(created)
    }

    pub async fn update(
        &self,
        operator: DefaultUser,
        id: Uuid,
        name: String,
    ) -> Result<Category, CtxError> {
        info!(
            user_id = %operator.id,
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
                .map_err(CtxError::from)
        }
        .await;

        match result {
            Ok(val) => {
                tx.commit().await.map_err(|e| {
                    error!(user_id=%operator.id, error=%e, "commit transaction failed");
                    CtxError::Internal
                })?;
                // 提交成功后，缓存需要失效
                self.cache.invalidate_all();
                self.post_page_cache_invalidator.invalidate_all();
                self.post_item_cache_invalidator.invalidate_all();
                Ok(val)
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    error!(user_id=%operator.id, error=%e, "rollback transaction failed");
                    CtxError::Internal
                })?;
                Err(e)
            }
        }
    }

    pub async fn delete(&self, operator: DefaultUser, id: Uuid) -> Result<(), CtxError> {
        info!(
            user_id = %operator.id,
            category_id = %id,
            "deleting category"
        );
        match self.repo.delete(&self.conn, id).await {
            Ok(true) => {
                // 删除成功后，缓存需要失效
                self.cache.invalidate_all();
                Ok(())
            }
            Ok(false) => Err(CtxError::NotFound(format!("category[{}]not exist", id))),
            Err(RepositoryError::ForeignKeyViolation(_)) => Err(CtxError::Conflict(format!(
                "category[{}] is referenced by posts and cannot be deleted",
                id
            ))),
            Err(e) => Err(e.into()),
        }
    }
}
