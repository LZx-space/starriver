use crate::repository::user::user_repository::UserRepositoryImpl;
use sea_orm::DatabaseConnection;
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_infrastructure::model::err::CodedErr;

pub struct UserApplication {
    repo: UserRepositoryImpl,
}

impl UserApplication {
    /// 新建
    pub fn new(conn: &'static DatabaseConnection) -> UserApplication {
        UserApplication {
            repo: UserRepositoryImpl::new(conn),
        }
    }

    pub async fn register_user(&self, username: &str, password: &str) -> Result<User, CodedErr> {
        // todo add publish register event
        let user = User::new_with_username_and_password(username, password)
            .map_err(|e| CodedErr::new_with_system_self_reason(e.to_string()))?;
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
