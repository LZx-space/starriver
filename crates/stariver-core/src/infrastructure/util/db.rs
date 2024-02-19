use std::env;

use async_static::async_static;
use sea_orm::{Database, DatabaseConnection};

async_static! {
    static ref CONN: DatabaseConnection = init_db_conn().await;
}

pub async fn db_conn() -> &'static DatabaseConnection {
    CONN.await
}

async fn init_db_conn() -> DatabaseConnection {
    let db_url = env::var("DB_URL").expect("DB_URL environment variable not set");
    Database::connect(db_url)
        .await
        .expect("create a DatabaseConnection failed")
}
