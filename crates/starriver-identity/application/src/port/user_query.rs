use sea_orm::ConnectionTrait;
use starriver_shared_base::error::QueryError;
use uuid::Uuid;

pub trait UserQuery {
    fn exists_by_email<C: ConnectionTrait>(
        &self,
        conn: &C,
        email: &str,
    ) -> impl Future<Output = Result<bool, QueryError>>;

    fn find_email_by_user_id<C: ConnectionTrait>(
        &self,
        conn: &C,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Option<String>, QueryError>>;
}
