use std::convert::Infallible;
use std::sync::Arc;

use crate::query::user_query_service::{DefaultUserQueryService, UserQueryService};
use crate::repository::user_repository::DefaultUserRepository;
use crate::user_dto::{EmailVerifyCmd, UserCmd};
use sea_orm::{DatabaseConnection, TransactionTrait};
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::{factory::UserFactory, specification::PasswordSpecification};
use starriver_infrastructure::service::cache_service::VerificationCodeCache;
use starriver_infrastructure::service::email_service::EmailClient;
use starriver_infrastructure::util::regex_patterns::Patterns;
use starriver_infrastructure::{
    error::ApiError,
    security::authentication::{
        _default_impl::{AuthenticatedUser, UsernamePasswordCredentials},
        core::authenticator::AuthenticationError,
    },
};
use tracing::{error, warn};

pub struct UserApplication {
    conn: DatabaseConnection,
    email_client: Arc<EmailClient>,
    verification_code_cache: Arc<VerificationCodeCache>,
    query: DefaultUserQueryService,
    repo: DefaultUserRepository<DatabaseConnection>,
    user_factory: UserFactory,
}

impl UserApplication {
    /// 新建
    pub fn new(
        conn: DatabaseConnection,
        email_client: Arc<EmailClient>,
        verification_code_cache: Arc<VerificationCodeCache>,
        patterns: Patterns,
    ) -> Self {
        let query = DefaultUserQueryService { conn: conn.clone() };
        let repo = DefaultUserRepository::new(conn.clone());
        let user_factory = UserFactory {
            email_regex: patterns.email.clone(),
        };
        Self {
            conn,
            email_client,
            verification_code_cache,
            query,
            repo,
            user_factory,
        }
    }

    pub async fn register_user(&self, cmd: UserCmd) -> Result<(), ApiError> {
        let email_code = cmd.email_code;
        let email = cmd.email.as_str();
        self.verification_code_cache
            .verify_email_by_verification_code(email, email_code.as_str())
            .await?;
        let user = self.user_factory.create_user(
            cmd.username.as_str(),
            cmd.password.as_str(),
            email,
            PasswordSpecification::default(),
        )?;
        self.repo.insert(user).await.map(|_| ())
    }

    pub async fn authenticate(
        &self,
        credentials: &UsernamePasswordCredentials,
    ) -> Result<AuthenticatedUser, AuthenticationError> {
        let username = credentials.username.as_str();
        let password = credentials.password.as_str();
        // 开启事务, update方法内部会再次查询获取副本以对比更新字段，当心事务等级
        let tx = self.conn.begin().await.map_err(|e| {
            error!("begin transaction error: {}", e);
            AuthenticationError::Unknown
        })?;
        let repo = DefaultUserRepository::new(tx);
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
                    email: user.email.to_string(),
                    authorities: vec![],
                }),
                Err(e) => {
                    // 更新用户
                    repo.update(user).await.map_err(|e| {
                        error!("update user error: {}", e);
                        AuthenticationError::Unknown
                    })?;
                    // 提交事务
                    repo.conn().commit().await.map_err(|e| {
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

    /// 发送邮箱验证邮件，永远不返回失败以防暴力核验邮箱
    pub async fn send_verification_email(&self, cmd: EmailVerifyCmd) -> Result<(), Infallible> {
        let email = cmd.email.as_str();
        match self.query.find_by_email(email).await {
            Ok(found) => {
                if found {
                    warn!("email already registered: {}", email);
                    return Ok(());
                }
                let verification_code = self
                    .verification_code_cache
                    .cache_email_verification_code(email)
                    .await;
                if let Err(e) = self
                    .email_client
                    .send_email_verification_mail(email, verification_code)
                    .await
                {
                    error!("send verification email error {}", e)
                }
                Ok(())
            }
            Err(e) => {
                error!("find user by email error {}", e);
                Ok(())
            }
        }
    }
}
