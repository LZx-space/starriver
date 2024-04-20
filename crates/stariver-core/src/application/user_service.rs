use sea_orm::DatabaseConnection;

use crate::domain::user::aggregate::User;
use crate::domain::user::repository::UserRepository;
use crate::infrastructure::model::err::CodedErr;
use crate::infrastructure::repository::user::user_repository::UserRepositoryImpl;

pub struct UserApplication {
    pub repo: UserRepositoryImpl,
}

impl UserApplication {
    /// 新建
    pub fn new(conn: &'static DatabaseConnection) -> UserApplication {
        UserApplication {
            repo: UserRepositoryImpl { conn },
        }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User, CodedErr> {
        let result = self.repo.find_by_username(username).await;
        result.map_err(|_err| CodedErr::new("B0000".to_string(), _err.to_string()))
    }
}
