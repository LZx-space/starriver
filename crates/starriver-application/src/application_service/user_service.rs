use crate::{
    query::user_query_service::{DefaultUserQueryService, UserQueryService},
    repository::user_repository::DefaultUserRepository,
    user_dto::SecurityUser,
};
use sea_orm::DatabaseConnection;
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_infrastructure::error::error::{ApiError, Cause};

pub struct UserApplication {
    repo: DefaultUserRepository,
    query_service: DefaultUserQueryService,
}

impl UserApplication {
    /// 新建
    pub fn new(conn: &'static DatabaseConnection) -> UserApplication {
        UserApplication {
            repo: DefaultUserRepository { conn },
            query_service: DefaultUserQueryService { conn },
        }
    }

    pub async fn register_user(&self, username: &str, password: &str) -> Result<User, ApiError> {
        // todo add publish register event
        let user = User::create_user(username, password)
            .map_err(|e| ApiError::new(Cause::ClientBadRequest, e.to_string()))?;
        self.repo.insert(user).await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<SecurityUser>, ApiError> {
        self.query_service.find_by_username(username).await
    }
}
