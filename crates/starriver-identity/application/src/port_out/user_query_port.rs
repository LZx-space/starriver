use starriver_shared_base::error::QueryError;
use uuid::Uuid;

pub trait UserQueryPort {
    fn exists_by_email(&self, email: &str) -> impl Future<Output = Result<bool, QueryError>>;

    fn find_email_by_user_id(
        &self,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Option<String>, QueryError>>;
}
