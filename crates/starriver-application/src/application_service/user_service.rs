use crate::repository::user_repository::DefaultUserRepository;
use sea_orm::{DatabaseConnection, TransactionTrait};
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
    conn: &'static DatabaseConnection,
}

impl UserApplication {
    /// 新建
    pub fn new(conn: &'static DatabaseConnection) -> UserApplication {
        UserApplication { conn }
    }

    pub async fn register_user(&self, username: &str, password: &str) -> Result<(), ApiError> {
        let user = UserFactory::create_user(username, password, PasswordSpecification::default())
            .map_err(|e| {
            error!("register user error: {}", e);
            ApiError::new(Cause::ClientBadRequest, e.to_string())
        })?;
        DefaultUserRepository::new(self.conn)
            .insert(user)
            .await
            .map(|_| ())
    }

    pub async fn authenticate(
        &self,
        credentials: &UsernamePasswordCredentials,
    ) -> Result<AuthenticatedUser, AuthenticationError> {
        let username = credentials.username.as_str();
        let password = credentials.password.as_str();
        // 开启事务, update方法内部会再次查询获取副本以对比更新字段，这依赖于事务等级，所以这里有潜在风险
        let tx = self.conn.begin().await.map_err(|e| {
            error!("begin transaction error: {}", e);
            AuthenticationError::Unknown
        })?;
        let repo = DefaultUserRepository::new(&tx);
        let opt = repo.find_by_username(username).await.map_err(|e| {
            // 用户名查不到用户不进这里，这里是异常才进
            error!("find by username error: {}", e);
            AuthenticationError::Unknown
        })?;
        if let Some(mut user) = opt {
            match user.authenticate_by_password(password, &PasswordSpecification::default()) {
                Ok(_) => Ok(AuthenticatedUser {
                    id: user.id,
                    username: username.to_string(),
                    authorities: vec![],
                }),
                Err(e) => {
                    repo.update(user).await.map_err(|e| {
                        error!("update user error: {}", e);
                        AuthenticationError::Unknown
                    })?;
                    // 提交事务
                    tx.commit().await.map_err(|e| {
                        error!("commit transaction error: {}", e);
                        AuthenticationError::Unknown
                    })?;
                    Err(e)
                }
            }
        } else {
            Err(AuthenticationError::UsernameNotFound)
        }
    }
}
