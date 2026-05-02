use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, TransactionError, TransactionTrait,
};
use starriver_domain::common_error::DomainError;

/// 表明既可以是普通链接，也可以是事务链接（使用完需要马上提交，且不能共享）
pub trait TransactionalConn: TransactionTrait + ConnectionTrait {}
impl<T: TransactionTrait + ConnectionTrait> TransactionalConn for T {}

pub trait TransactionalRepository {
    fn with_transaction_connection(&self, conn: DatabaseTransaction) -> Self;

    fn transaction_connection(self) -> DatabaseTransaction;
}

pub struct TransactionManager<R> {
    conn: DatabaseConnection,
    inner: R,
}

impl<REPO> TransactionManager<REPO>
where
    REPO: TransactionalRepository,
{
    pub async fn execute<F, Fut, R>(&self, f: F) -> Result<R, TransactionError<DomainError>>
    where
        F: FnOnce(&REPO) -> Fut + Send + 'static,
        Fut: Future<Output = Result<R, TransactionError<DomainError>>> + Send,
        R: Send + 'static,
    {
        let tx = self.conn.begin().await?;
        let repo = self.inner.with_transaction_connection(tx);
        match f(&repo).await {
            Ok(result) => {
                let tx = repo.transaction_connection();
                tx.commit().await?;
                Ok(result)
            }
            Err(error) => {
                let tx = repo.transaction_connection();
                tx.rollback().await?;
                Err(error)
            }
        }
    }

    pub fn inner(&self) -> &REPO {
        &self.inner
    }
}
