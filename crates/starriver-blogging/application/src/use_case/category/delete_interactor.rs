use starriver_shared_base::authentication::principal::DefaultUser;
use starriver_shared_base::cache::Cache;
use starriver_shared_base::db::Connection;
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
    R: CategoryRepository<Conn>,
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
            Err(e) => Err(e.into()),
        }
    }
}
