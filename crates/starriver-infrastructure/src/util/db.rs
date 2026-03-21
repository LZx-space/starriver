use sea_orm::{ConnectionTrait, Database, DatabaseConnection, TransactionTrait};
use std::env;
use std::sync::OnceLock;

static DB_CONN: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn db_conn() -> &'static DatabaseConnection {
    if let Some(conn) = DB_CONN.get() {
        return conn;
    }
    let db_url = env::var("DB_URL").expect("DB_URL environment variable not set");
    let conn = Database::connect(db_url)
        .await
        .expect("create a DatabaseConnection failed");
    DB_CONN.get_or_init(|| conn)
}

/// 表明既可以是普通链接，也可以是事务链接（使用完需要马上提交，且不能共享）
pub trait TransactionalConn: TransactionTrait + ConnectionTrait + Send + Sync {}
impl<T: TransactionTrait + ConnectionTrait + Send + Sync> TransactionalConn for T {}
