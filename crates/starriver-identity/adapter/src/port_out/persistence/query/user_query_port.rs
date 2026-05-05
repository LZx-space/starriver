use sea_orm::DatabaseConnection;
use starriver_identity_application::{
    common::error::UserQueryError, port_out::user_query_port::UserQueryPort,
};

pub struct DefaultUserQueryPort {
    pub conn: DatabaseConnection,
}

impl UserQueryPort for DefaultUserQueryPort {
    async fn exists_by_email(&self, email: &str) -> Result<bool, UserQueryError> {
        todo!()
    }
}
