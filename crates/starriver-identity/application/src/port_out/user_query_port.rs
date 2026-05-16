use starriver_shared_base::error::QueryError;

pub trait UserQueryPort {
    fn exists_by_email(&self, email: &str) -> impl Future<Output = Result<bool, QueryError>>;
}
