use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::authentication::principal::DefaultUser;
use starriver_shared_base::cache::Cache;
use starriver_shared_base::db::Connection;
use tracing::info;

use crate::dto::category_dto::output::CategoryDetailDto;
use crate::error::CtxError;
use crate::port::category_cache::CategoryListKey;
use crate::port::category_repository::CategoryRepository;

pub struct CreateInteractor<Conn, R, C> {
    conn: Conn,
    repo: R,
    cache: C,
}

impl<Conn, R, C> CreateInteractor<Conn, R, C>
where
    Conn: Connection,
    R: CategoryRepository<Conn> + CategoryRepository<<Conn as Connection>::Transaction>,
    C: Cache<CategoryListKey, Vec<CategoryDetailDto>>,
{
    pub fn new(conn: Conn, repo: R, cache: C) -> Self {
        Self { conn, repo, cache }
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
}
