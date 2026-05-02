use std::convert::Infallible;

use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use starriver_base::dto::user_dto::req::{EmailVerifyCmd, UserCmd};
use starriver_base::query::user_query_service::DefaultUserQueryService;
use starriver_base::query::user_query_service::UserQueryService;
use starriver_base::repository::user_repository::DefaultUserRepository;
use starriver_base::security::password_encoder::Argon2PasswordEncoder;
use starriver_base::service::cache_service::VerificationCodeCache;
use starriver_base::service::config_service::UserPolicy as UserPolicyConfig;
use starriver_base::service::email_service::EmailClient;
use starriver_base::util::regex_patterns::Patterns;
use starriver_base::{
    error::ApiError,
    security::authentication::{
        _default_impl::{AuthenticatedUser, UsernamePasswordCredentials},
        core::authenticator::AuthenticationError,
    },
};
use starriver_domain::common_error::DomainError;
use starriver_domain::common_model::Revision;
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::{factory::UserFactory, policy::UserLockPolicy};
use tracing::{error, info, warn};

pub struct UserApplication {
    conn: DatabaseConnection,
    email_client: EmailClient,
    verification_code_cache: VerificationCodeCache,
    factory: UserFactory,
    user_lock_policy: UserLockPolicy,
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
        user_policy_cfg: UserPolicyConfig,
    ) -> Self {
        let factory = UserFactory::new(patterns.email, patterns.username, patterns.password);
        let user_lock_policy = UserLockPolicy::new(
            user_policy_cfg.bad_password_window_mins,
            user_policy_cfg.max_bad_password_attempts,
        );

        let query = DefaultUserQueryService { conn: conn.clone() };
        let repo = DefaultUserRepository::new(conn.clone(), factory.clone());

        Self {
            conn,
            email_client,
            verification_code_cache,
            factory,
            user_lock_policy,
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
        let user = self
            .factory
            .create_user(
                cmd.username.as_str(),
                cmd.password.as_str(),
                email,
                &self.password_encoder,
            )
            .map_err(|e| ApiError::with_bad_request(e.to_string()))?;
        self.repo
            .insert(user)
            .await
            .map_err(|e| ApiError::with_bad_request(e.to_string()))
            .map(|_| ())
    }

    pub async fn authenticate(
        &self,
        credentials: &UsernamePasswordCredentials,
    ) -> Result<AuthenticatedUser, AuthenticationError> {
        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            AuthenticationError::InnerError
        })?;
        let tx_repo = DefaultUserRepository::new(tx, self.factory.clone());
        match self.transaction_authenticate(credentials, &tx_repo).await {
            Ok(user) => {
                tx_repo.conn().commit().await.map_err(|e| {
                    error!(error = %e, "commit transaction failed");
                    AuthenticationError::InnerError
                })?;
                info!(username = %credentials.username, "user authenticated successfully");
                Ok(user)
            }
            Err(AuthenticationError::InnerError) => {
                tx_repo.conn().rollback().await.map_err(|e| {
                    error!(error = %e, "rollback transaction failed");
                    AuthenticationError::InnerError
                })?;
                Err(AuthenticationError::InnerError)
            }
            Err(e) => {
                tx_repo.conn().commit().await.map_err(|e| {
                    error!(error = %e, "commit transaction after auth failure failed");
                    AuthenticationError::InnerError
                })?;
                Err(e)
            }
        }
    }

    /// 发送邮箱验证邮件，永远不返回失败以防暴力核验邮箱
    pub async fn send_verification_email(&self, cmd: EmailVerifyCmd) -> Result<(), Infallible> {
        let email = cmd.email.as_str();
        match self.query.exists_by_email(email).await {
            Ok(found) => {
                if found {
                    warn!(email = %email, "email already registered, skipping verification");
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
                    error!(email = %email, error = %e, "send verification email failed");
                }
                Ok(())
            }
            Err(e) => {
                error!(email = %email, error = %e, "find user by email failed");
                Ok(())
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    async fn transaction_authenticate(
        &self,
        credentials: &UsernamePasswordCredentials,
        tx_repo: &DefaultUserRepository<DatabaseTransaction>,
    ) -> Result<AuthenticatedUser, AuthenticationError> {
        let username = credentials.username.as_str();
        let password = credentials.password.as_str();
        let mut user = tx_repo
            .find_by_username(username)
            .await
            .map_err(|e| {
                error!(username = %username, error = %e, "find by username failed");
                AuthenticationError::InnerError
            })?
            .ok_or(AuthenticationError::UsernameNotFound)?;
        let original = user.clone();
        match user.authenticate_by_password(
            password,
            &self.user_lock_policy,
            &self.password_encoder,
        ) {
            Ok(_) => Ok(AuthenticatedUser {
                id: user.id().to_owned(),
                username: username.to_string(),
                email: user.email().to_string(),
                authorities: vec![],
            }),
            Err(DomainError::BadPassword) => {
                if user.state() == &starriver_domain::user::value_object::UserState::Locked {
                    warn!(
                        user.id = %user.id(),
                        username = %username,
                        "user locked due to too many bad password attempts"
                    );
                }
                let user = Revision::new(original, user);
                tx_repo.update(user).await.map_err(|e| {
                    error!(username = %username, error = %e, "update user after bad password failed");
                    AuthenticationError::InnerError
                })?;
                Err(AuthenticationError::BadPassword)
            }
            Err(e) => {
                error!(username = %username, error = %e, "authenticate by password failed");
                Err(AuthenticationError::InnerError)
            }
        }
    }
}
