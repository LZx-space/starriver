use crate::domain::user::aggregate::User;
use crate::domain::user::repository::UserRepository;
use crate::infrastructure::model::err::CodedErr;

pub struct UserApplication<T> {
    pub repo: T,
}

impl<T> UserApplication<T>
where
    T: UserRepository,
{
    /// 新建
    pub fn new(repo: T) -> UserApplication<T> {
        UserApplication { repo }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, CodedErr> {
        let result = self.repo.find_by_username(username).await;
        result.map_err(|_err| CodedErr::new("B0000".to_string(), _err.to_string()))
    }
}
