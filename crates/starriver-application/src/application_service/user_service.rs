use crate::repository::user_repository::DefaultUserRepository;
use sea_orm::DatabaseConnection;
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_infrastructure::error::error::{ApiError, Cause};

#[derive(Clone)]
pub struct UserApplication {
    repo: DefaultUserRepository,
}

impl UserApplication {
    /// 新建
    pub fn new(conn: &'static DatabaseConnection) -> UserApplication {
        UserApplication {
            repo: DefaultUserRepository { conn },
        }
    }

    pub async fn register_user(&self, username: &str, password: &str) -> Result<User, ApiError> {
        // todo add publish register event
        let user = User::create_user(username, password)
            .map_err(|e| ApiError::new(Cause::ClientBadRequest, e.to_string()))?;
        self.repo.insert(user).await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        self.repo.find_by_username(username).await
    }
}
