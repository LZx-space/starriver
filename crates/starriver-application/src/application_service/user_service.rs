use crate::repository::user_repository::DefaultUserRepository;
use sea_orm::DatabaseConnection;
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::{factory::UserFactory, specification::PasswordSpecification};
use starriver_infrastructure::{
    error::{ApiError, Cause},
    security::authentication::{
        _default_impl::{AuthenticatedUser, UsernamePasswordCredentials},
        core::authenticator::AuthenticationError,
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
        credentials: &UsernamePasswordCredentials,
    ) -> Result<AuthenticatedUser, AuthenticationError> {
        let username = credentials.username.as_str();
        let password = credentials.password.as_str();
        let opt = self.repo.find_by_username(username).await.map_err(|e| {
            // 用户名查不到用户不进这里，这里是异常才进
            error!("find by username error: {}", e);
            AuthenticationError::Unknown
        })?;
        match opt {
            Some(mut user) => {
                let auth =
                    user.authenticate_by_password(password, &PasswordSpecification::default());
                if let Some(state) = auth.state {
                    self.repo.update_auth_pwd_state(state).await.map_err(|e| {
                        error!("update user error: {}", e);
                        AuthenticationError::Unknown
                    })?;
                }
                if let Some(error) = auth.error {
                    Err(error)
                } else {
                    Ok(AuthenticatedUser {
                        id: user.id,
                        username: username.to_string(),
                        authorities: vec![],
                    })
                }
            }
            None => Err(AuthenticationError::UsernameNotFound),
        }
    }
}
