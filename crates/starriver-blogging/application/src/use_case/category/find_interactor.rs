use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::db::Connection;
use uuid::Uuid;

use crate::error::CtxError;
use crate::port::category_repository::CategoryRepository;

pub struct FindInteractor<Conn, R> {
    conn: Conn,
    repo: R,
}

impl<Conn, R> FindInteractor<Conn, R>
where
    Conn: Connection,
    R: CategoryRepository<Conn> + CategoryRepository<<Conn as Connection>::Transaction>,
{
    pub fn new(conn: Conn, repo: R) -> Self {
        Self { conn, repo }
    }

    pub async fn find(&self, id: Uuid) -> Result<Category, CtxError> {
        self.repo
            .find_by_id(&self.conn, id)
            .await?
            .ok_or_else(|| CtxError::NotFound(format!("Category [{}] not found", id)))
    }
}
