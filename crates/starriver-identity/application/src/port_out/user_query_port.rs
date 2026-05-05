use crate::common::error::UserQueryError;

pub trait UserQueryPort {
    fn exists_by_email(&self, email: &str) -> impl Future<Output = Result<bool, UserQueryError>>;
}
