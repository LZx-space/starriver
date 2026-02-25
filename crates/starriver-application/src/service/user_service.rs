use crate::repository::user::user_repository::UserRepositoryImpl;
use sea_orm::DatabaseConnection;
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_infrastructure::error::error::{AppError, Cause};
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

    pub async fn register_user(&self, username: &str, password: &str) -> Result<User, AppError> {
        // todo add publish register event
        let user = User::create_user(username, password)
            .map_err(|e| AppError::new(Cause::ClientBadRequest, e.to_string()))?;
        self.repo
            .insert(user)
            .await
            .map_err(|e| AppError::new(Cause::DbError, e.to_string()))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        self.repo
            .find_by_username(username)
            .await
            .map_err(|err| AppError::new(Cause::DbError, err.to_string()))
    }
}
