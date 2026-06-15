use starriver_shared_base::{error::QueryError, repository::Executor};
use uuid::Uuid;

pub trait UserQuery<T: Executor> {
    fn exists_by_email(
        &self,
        conn: &T,
        email: &str,
    ) -> impl Future<Output = Result<bool, QueryError>>;

    fn find_email_by_user_id(
        &self,
        conn: &T,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Option<String>, QueryError>>;
}
