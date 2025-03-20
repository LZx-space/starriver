use sea_orm::DatabaseConnection;

use crate::domain::user::aggregate::User;
use crate::domain::user::repository::UserRepository;
use crate::infrastructure::model::err::CodedErr;
use crate::infrastructure::repository::user::user_repository::UserRepositoryImpl;

pub struct UserApplication {
    repo: UserRepositoryImpl,
}

impl UserApplication {
    /// 新建
    pub(crate) fn new(conn: &'static DatabaseConnection) -> UserApplication {
        UserApplication {
            repo: UserRepositoryImpl::new(conn),
        }
    }

    pub async fn insert(&self, user: User) -> Result<User, CodedErr> {
        self.repo
            .insert(user)
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, CodedErr> {
        self.repo
            .find_by_username(username)
            .await
            .map_err(|err| CodedErr::new("B0000".to_string(), err.to_string()))
    }
}
