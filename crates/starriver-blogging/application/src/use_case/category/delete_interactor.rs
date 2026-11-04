use starriver_shared_base::authentication::principal::DefaultUser;
use starriver_shared_base::cache::Cache;
use starriver_shared_base::db::Connection;
use starriver_shared_base::error::RepositoryError;
use tracing::info;
use uuid::Uuid;

use crate::dto::category_dto::output::CategoryDetailDto;
use crate::error::CtxError;
use crate::port::category_cache::CategoryListKey;
use crate::port::category_repository::CategoryRepository;

pub struct DeleteInteractor<Conn, R, C> {
    conn: Conn,
    repo: R,
    cache: C,
}

impl<Conn, R, C> DeleteInteractor<Conn, R, C>
where
    Conn: Connection,
    R: CategoryRepository<Conn> + CategoryRepository<<Conn as Connection>::Transaction>,
    C: Cache<CategoryListKey, Vec<CategoryDetailDto>>,
{
    pub fn new(conn: Conn, repo: R, cache: C) -> Self {
        Self { conn, repo, cache }
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
            Ok(false) => Err(CtxError::NotFound(format!("Category [{}] not found", id))),
            Err(RepositoryError::ForeignKeyViolation(_)) => Err(CtxError::Conflict(format!(
                "Category [{}] is still referenced by posts and cannot be deleted",
                id
            ))),
            Err(e) => Err(e.into()),
        }
    }
}
