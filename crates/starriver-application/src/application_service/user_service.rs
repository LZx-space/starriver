use std::convert::Infallible;

use crate::query::user_query_service::{DefaultUserQueryService, UserQueryService};
use crate::repository::user_repository::DefaultUserRepository;
use crate::user_dto::req::{EmailVerifyCmd, UserCmd};
use sea_orm::{DatabaseConnection, TransactionTrait};
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::{factory::UserFactory, policy::UserPolicy};
use starriver_infrastructure::security::password_encoder::Argon2PasswordEncoder;
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
    email_client: EmailClient,
    verification_code_cache: VerificationCodeCache,
    factory: UserFactory,
    password_encoder: Argon2PasswordEncoder,
    query: DefaultUserQueryService,
    repo: DefaultUserRepository<DatabaseConnection>,
}

impl UserApplication {
    /// 新建
    pub fn new(
        conn: DatabaseConnection,
        email_client: EmailClient,
        verification_code_cache: VerificationCodeCache,
        patterns: Patterns,
    ) -> Self {
        let factory = UserFactory { patterns };

        let query = DefaultUserQueryService { conn: conn.clone() };
        let repo = DefaultUserRepository::new(conn.clone(), factory.clone());

        Self {
            conn,
            email_client,
            verification_code_cache,
            factory,
            password_encoder: Argon2PasswordEncoder::default(),
            query,
            repo,
        }
    }

    pub async fn register_user(&self, cmd: UserCmd) -> Result<(), ApiError> {
        let email_code = cmd.email_code;
        let email = cmd.email.as_str();
        self.verification_code_cache
            .verify_email_by_verification_code(email, email_code.as_str())
            .await?;
        let user = self.factory.create_user(
            cmd.username.as_str(),
            cmd.password.as_str(),
            email,
            &self.password_encoder,
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
        let repo = DefaultUserRepository::new(tx, self.factory.clone());
        let opt = repo.find_by_username(username).await.map_err(|e| {
            // 用户名查不到用户不进这里，这里是异常才进
            error!("find by username error: {}", e);
            AuthenticationError::Unknown
        })?;
        if let Some(mut user) = opt {
            match user.authenticate_by_password(
                password,
                &UserPolicy::default(),
                &self.password_encoder,
            ) {
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
