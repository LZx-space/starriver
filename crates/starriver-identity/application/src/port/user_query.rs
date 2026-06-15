use starriver_shared_base::{error::QueryError, repository::Executor};
use uuid::Uuid;

pub trait UserQuery<C: Executor> {
    fn exists_by_email(
        &self,
        conn: &C,
        email: &str,
    ) -> impl Future<Output = Result<bool, QueryError>>;

    fn find_email_by_user_id(
        &self,
        conn: &C,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Option<String>, QueryError>>;
}
