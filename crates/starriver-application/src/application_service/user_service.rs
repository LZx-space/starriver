use crate::repository::user_repository::DefaultUserRepository;
use sea_orm::DatabaseConnection;
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::{factory::UserFactory, specification::PasswordSpecification};
use starriver_infrastructure::{
    error::error::{ApiError, Cause},
    security::authentication::{
        core::authenticator::AuthenticationError,
        username_password_authentication::{AuthenticatedUser, UsernamePasswordCredential},
    },
};
use tracing::error;

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

    pub async fn register_user(&self, username: &str, password: &str) -> Result<(), ApiError> {
        let user = UserFactory::create_user(username, password, PasswordSpecification::default())
            .map_err(|e| {
            error!("register user error: {}", e);
            ApiError::new(Cause::ClientBadRequest, e.to_string())
        })?;
        self.repo.insert(user).await.map(|_| ())
    }

    pub async fn authenticate(
        &self,
        credential: &UsernamePasswordCredential,
    ) -> Result<AuthenticatedUser, AuthenticationError> {
        let username = credential.username.as_str();
        let password = credential.password.as_str();
        let opt = self.repo.find_by_username(username).await.map_err(|e| {
            // 用户名查不到用户不进这里，这里是异常才进
            error!("find by username error: {}", e);
            AuthenticationError::Unknown
        })?;
        match opt {
            Some(mut user) => {
                user.authenticate_by_password(password)?;
                Ok(AuthenticatedUser {
                    username: username.to_string(),
                    password: "".to_string(),
                    authorities: vec![],
                })
            }
            None => return Err(AuthenticationError::UsernameNotFound),
        }
    }
}
