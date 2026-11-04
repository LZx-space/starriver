use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::cache::{Cache, Invalidator};
use starriver_shared_base::db::Connection;
use starriver_shared_base::{authentication::principal::DefaultUser, db::Revision};
use tracing::info;
use uuid::Uuid;

use crate::dto::category_dto::output::CategoryDetailDto;
use crate::error::CtxError;
use crate::port::category_cache::CategoryListKey;
use crate::port::category_repository::CategoryRepository;
use crate::port::post_cache::PostPageKey;

pub struct UpdateInteractor<Conn, R, C, PPI, PII> {
    conn: Conn,
    repo: R,
    cache: C,
    post_page_cache_invalidator: PPI,
    post_item_cache_invalidator: PII,
}

impl<Conn, R, C, PPI, PII> UpdateInteractor<Conn, R, C, PPI, PII>
where
    Conn: Connection,
    R: CategoryRepository<Conn> + CategoryRepository<<Conn as Connection>::Transaction>,
    C: Cache<CategoryListKey, Vec<CategoryDetailDto>>,
    PPI: Invalidator<PostPageKey>,
    PII: Invalidator<Uuid>,
{
    pub fn new(
        conn: Conn,
        repo: R,
        cache: C,
        post_page_cache_invalidator: PPI,
        post_item_cache_invalidator: PII,
    ) -> Self {
        Self {
            conn,
            repo,
            cache,
            post_page_cache_invalidator,
            post_item_cache_invalidator,
        }
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

        let category = self.repo.find_by_id(&self.conn, id).await?;
        let Some(mut category) = category else {
            return Err(CtxError::NotFound(format!("Category [{}] not found", id)));
        };
        let original = category.clone();
        category.update(name)?;
        let updated = self
            .repo
            .update(&self.conn, Revision::new(original, category))
            .await?;

        // 更新成功后，缓存需要失效
        self.cache.invalidate_all();
        self.post_page_cache_invalidator.invalidate_all();
        self.post_item_cache_invalidator.invalidate_all();
        Ok(updated)
    }
}
