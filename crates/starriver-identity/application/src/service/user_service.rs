use std::{convert::Infallible, sync::Arc};

use starriver_identity_domain::{
    authentication_service::AuthenticationService,
    error::DomainError,
    password_encoder::PasswordEncoder,
    security_event::{
        entity::SecurityEvent, repository::SecurityEventRepository, value_object::SecurityEventType,
    },
    user::{
        factory::UserFactory, policy::BadPasswordPolicy, repository::UserRepository,
        value_object::UserState,
    },
};
use starriver_shared_base::{
    authentication::UsernamePasswordCredentials, error::RepositoryError,
    middleware::authentication::core::error::AuthenticationError, regex_patterns::Patterns,
    repository::Revision,
};
use time::{Duration, OffsetDateTime};
use tracing::{error, info, warn};

use crate::{
    dto::user_dto::{
        req::{EmailActiveCmd, EmailVerifyCmd, UserActiveCmd, UserCmd},
        res::UserDetail,
    },
    error::CtxError,
    port_out::{email_verification_port::EmailVerificationPort, user_query_port::UserQueryPort},
};

pub struct UserApplicationService<UQP, UREPO, SREPO, VCP, PE> {
    user_query: UQP,
    user_repo: UREPO,
    security_event_repo: SREPO,
    verification_code_port: VCP,
    factory: UserFactory<PE>,
    auth_service: AuthenticationService<PE>,
}

impl<UQP, UREPO, SREPO, VCP, PE> UserApplicationService<UQP, UREPO, SREPO, VCP, PE>
where
    UQP: UserQueryPort,
    UREPO: UserRepository,
    VCP: EmailVerificationPort,
    PE: PasswordEncoder + Clone,
    SREPO: SecurityEventRepository,
{
    /// 新建
    pub fn new(
        user_query_port: UQP,
        user_repo: UREPO,
        security_event_repo: SREPO,
        verification_code_port: VCP,
        patterns: Patterns,
        bad_password_policy: BadPasswordPolicy,
        password_encoder: Arc<PE>,
    ) -> Self {
        let factory = UserFactory::new(
            patterns.email,
            patterns.username,
            patterns.password,
            password_encoder.clone(),
        );
        let auth_service = AuthenticationService::new(bad_password_policy, password_encoder);

        Self {
            user_query: user_query_port,
            user_repo,
            security_event_repo,
            verification_code_port,
            factory,
            auth_service,
        }
    }

    /// 发送邮箱验证邮件，永远不返回失败以防暴力核验邮箱
    pub async fn send_register_email(&self, cmd: EmailVerifyCmd) -> Result<(), Infallible> {
        let email = cmd.email.as_str();
        match self.user_query.exists_by_email(email).await {
            Ok(found) => {
                if found {
                    warn!(email=%email, "email already registered, skipping verification");
                    return Ok(());
                }
                if let Err(e) = self.verification_code_port.send_code(email).await {
                    error!(email=%email, error=%e, "send verification email failed");
                }
                Ok(())
            }
            Err(e) => {
                error!(email=%email, error=%e, "find user by email failed");
                Ok(())
            }
        }
    }

    pub async fn register_user(&self, cmd: UserCmd) -> Result<(), CtxError> {
        let email_code = cmd.email_code.as_str();
        let email = cmd.email.as_str();
        let matches = self
            .verification_code_port
            .validate_code(email, email_code)
            .await
            .inspect_err(|e| info!(email=%email, error=%e, "rigister user validate code failed"))?;
        if !matches {
            return Err(CtxError::InvalidInput("invalid email code".to_string()));
        }
        let user = self
            .factory
            .create_user(cmd.username.as_str(), cmd.password.as_str(), email)
            .inspect_err(|e| info!(email=%email, error=%e, "rigister user create user failed"))?;
        self.user_repo
            .insert(user)
            .await
            .inspect_err(|e| error!(email=%email, error=%e, "repository insert user failed"))?;
        Ok(())
    }

    /// 发送用户激活邮件，永远不返回失败以防暴力核验邮箱
    pub async fn send_active_email(&self, cmd: EmailActiveCmd) -> Result<(), Infallible> {
        let email = cmd.email.as_str();
        match self.user_query.find_email_by_user_id(cmd.user_id).await {
            Ok(found) => {
                if found.is_some_and(|e| e != email) {
                    warn!(email=%email, "incorrect email for user");
                    return Ok(());
                }
                if let Err(e) = self.verification_code_port.send_code(email).await {
                    error!(email=%email, error=%e, "send active email failed");
                }
                Ok(())
            }
            Err(e) => {
                error!(email=%email, error=%e, "find email by user id failed");
                Ok(())
            }
        }
    }

    pub async fn activate_user(
        &self,
        username: String,
        cmd: UserActiveCmd,
    ) -> Result<(), CtxError> {
        let email_code = cmd.email_code.as_str();
        match self.user_repo.find_by_username(username.clone()).await {
            Ok(found) => {
                if let Some(mut found) = found {
                    let email = found.email().as_str();
                    let matches = self
                        .verification_code_port
                        .validate_code(email, email_code)
                        .await
                        .inspect_err(
                            |e| info!(email=%email, error=%e, "active user validate code failed"),
                        )?;
                    if !matches {
                        return Err(CtxError::InvalidInput("invalid email code".to_string()));
                    }
                    let original = found.clone();
                    found.activate();
                    self.user_repo
                        .update(Revision::new(original, found))
                        .await?;
                } else {
                    warn!(username=%username, "user not found");
                }
                Ok(())
            }
            Err(e) => {
                error!(user_id=%username, error=%e, "find user by id failed");
                Err(CtxError::Internal(e.to_string()))
            }
        }
    }

    pub async fn authenticate(
        &self,
        credentials: &UsernamePasswordCredentials,
    ) -> Result<UserDetail, AuthenticationError> {
        let username = credentials.username.as_str();
        let password = credentials.password.as_str();
        let user = self
            .user_repo
            .find_by_username(username.to_string())
            .await
            .map_err(mapping_error())?;

        let Some(mut user) = user else {
            info!(username=%username, "user not found");
            return Err(AuthenticationError::BadPassword); // 避免刻意查询账户是否存在
        };

        let result = self.auth_service.authenticate(&user, password);

        match result {
            Ok(()) => {
                let fields = user.dissolve();
                Ok(UserDetail {
                    id: fields.0,
                    username: fields.1.to_string(),
                    email: fields.3.to_string(),
                })
            }
            Err(domain_err) => {
                info!(error=%domain_err, "authentication failed");
                // BadPassword 时：记录事件 + 判断锁定
                if matches!(domain_err, DomainError::BadPassword) {
                    info!("handle bad password event");
                    let event = SecurityEvent::new(
                        *user.id(),
                        SecurityEventType::TryLoginWithBadPwd,
                        "bad password attempt",
                    );
                    self.security_event_repo
                        .insert(event)
                        .await
                        .map_err(mapping_error())?;

                    let since = OffsetDateTime::now_utc().saturating_sub(Duration::minutes(
                        self.auth_service.policy().window_minutes as i64,
                    ));
                    let events = self
                        .security_event_repo
                        .find_by_user_id_since(
                            *user.id(),
                            SecurityEventType::TryLoginWithBadPwd,
                            since,
                        )
                        .await
                        .map_err(mapping_error())?;

                    let original = user.clone();
                    self.auth_service.check_and_lock_user(&mut user, &events);

                    if matches!(user.state(), UserState::Locked) {
                        info!(user_id=%user.id(), "user locked");
                        self.user_repo
                            .update(Revision::new(original, user))
                            .await
                            .map_err(mapping_error())?;
                    }
                }

                // 统一转换
                Err(match domain_err {
                    DomainError::UserLocked => AuthenticationError::UserLocked,
                    DomainError::UserDisabled => AuthenticationError::UserDisabled,
                    DomainError::BadPassword => AuthenticationError::BadPassword,
                    _ => AuthenticationError::InnerError {
                        message: domain_err.to_string(),
                        source: Box::new(domain_err),
                    },
                })
            }
        }
    }
}

fn mapping_error() -> impl FnOnce(RepositoryError) -> AuthenticationError {
    |e| {
        error!(error=%e, "handle bad password event failed");
        AuthenticationError::InnerError {
            message: e.to_string(),
            source: Box::new(e),
        }
    }
}
