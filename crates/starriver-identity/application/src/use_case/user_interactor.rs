use std::convert::Infallible;

use sea_orm::{ConnectionTrait, TransactionTrait};
use starriver_identity_domain::{
    authentication_service::AuthenticationDomainService,
    error::DomainError,
    password_encoder::PasswordEncoder,
    security_event::{entity::SecurityEvent, value_object::SecurityEventType},
    user::{factory::UserFactory, value_object::UserState},
};
use starriver_shared_base::{
    authentication::UsernamePasswordCredentials, error::RepositoryError,
    middleware::authentication::core::error::AuthenticationError, repository::Revision,
};
use time::{Duration, OffsetDateTime};
use tracing::{error, info, warn};

use crate::{
    dto::user_dto::{
        req::{EmailActiveCmd, EmailVerifyCmd, UserActiveCmd, UserCmd},
        res::UserDetail,
    },
    error::CtxError,
    port::{
        email_verification_service::EmailVerificationService,
        security_event_repository::SecurityEventRepository, user_query::UserQuery,
        user_repository::UserRepository,
    },
};

pub struct UserApplicationService<Conn, UQP, UREPO, SREPO, VCP, PE> {
    conn: Conn,
    user_query: UQP,
    user_repo: UREPO,
    security_event_repo: SREPO,
    verification_code_service: VCP,
    user_factory: UserFactory<PE>,
    auth_service: AuthenticationDomainService<PE>,
}

impl<Conn, UQP, UREPO, SREPO, VCP, PE> UserApplicationService<Conn, UQP, UREPO, SREPO, VCP, PE>
where
    Conn: ConnectionTrait + TransactionTrait,
    UQP: UserQuery + Sync,
    UREPO: UserRepository + Sync,
    VCP: EmailVerificationService + Send + Sync,
    PE: PasswordEncoder + Send + Sync,
    SREPO: SecurityEventRepository + Sync,
{
    /// 新建
    pub fn new(
        conn: Conn,
        user_query: UQP,
        user_repo: UREPO,
        security_event_repo: SREPO,
        verification_code_port: VCP,
        user_factory: UserFactory<PE>,
        auth_service: AuthenticationDomainService<PE>,
    ) -> Self {
        Self {
            conn,
            user_query,
            user_repo,
            security_event_repo,
            verification_code_service: verification_code_port,
            user_factory,
            auth_service,
        }
    }

    /// 发送邮箱验证邮件，永远不返回失败以防暴力核验邮箱
    pub async fn send_register_email(&self, cmd: EmailVerifyCmd) -> Result<(), Infallible> {
        let email = cmd.email.as_str();
        match self.user_query.exists_by_email(&self.conn, email).await {
            Ok(found) => {
                if found {
                    warn!(email=%email, "email already registered, skipping verification");
                    return Ok(());
                }
                if let Err(e) = self.verification_code_service.send_code(email).await {
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
            .verification_code_service
            .validate_code(email, email_code)
            .await
            .inspect_err(|e| info!(email=%email, error=%e, "rigister user validate code failed"))?;
        if !matches {
            return Err(CtxError::InvalidInput("invalid email code".to_string()));
        }
        let user = self
            .user_factory
            .create_user(cmd.username.as_str(), cmd.password.as_str(), email)
            .inspect_err(|e| info!(email=%email, error=%e, "rigister user create user failed"))?;
        self.user_repo
            .insert(&self.conn, user)
            .await
            .inspect_err(|e| error!(email=%email, error=%e, "repository insert user failed"))?;
        Ok(())
    }

    /// 发送用户激活邮件，永远不返回失败以防暴力核验邮箱
    pub async fn send_active_email(&self, cmd: EmailActiveCmd) -> Result<(), Infallible> {
        let email = cmd.email.as_str();
        match self
            .user_query
            .find_email_by_user_id(&self.conn, cmd.user_id)
            .await
        {
            Ok(found) => {
                if found.is_some_and(|e| e != email) {
                    warn!(email=%email, "incorrect email for user");
                    return Ok(());
                }
                if let Err(e) = self.verification_code_service.send_code(email).await {
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

        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;
        let result = match self.user_repo.find_by_username(&tx, &username).await {
            Ok(found) => {
                if let Some(mut found) = found {
                    let email = found.email().as_str();
                    let matches = self
                        .verification_code_service
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
                        .update(&self.conn, Revision::new(original, found))
                        .await?;
                } else {
                    warn!(username=%username, "user not found");
                }
                Ok(())
            }
            Err(e) => {
                error!(user_id=%username, error=%e, "find user by id failed");
                Err(CtxError::Internal)
            }
        };
        match result {
            Ok(val) => {
                tx.commit().await.map_err(|e| {
                    error!(user_id=%username, error=%e, "commit transaction failed");
                    CtxError::Internal
                })?;
                Ok(val)
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    error!(user_id=%username, error=%e, "rollback transaction failed");
                    CtxError::Internal
                })?;
                Err(e)
            }
        }
    }

    pub async fn authenticate(
        &self,
        credentials: &UsernamePasswordCredentials,
    ) -> Result<UserDetail, AuthenticationError> {
        let username = credentials.username.as_str();
        let password = credentials.password.as_str();

        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            AuthenticationError::InnerError {
                message: e.to_string(),
                source: Box::new(e),
            }
        })?;
        let result = async {
            let user = self
                .user_repo
                .find_by_username(&self.conn, username)
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
                            .insert(&self.conn, event)
                            .await
                            .map_err(mapping_error())?;

                        let since = OffsetDateTime::now_utc().saturating_sub(Duration::minutes(
                            self.auth_service.policy().window_minutes as i64,
                        ));
                        let events = self
                            .security_event_repo
                            .find_by_user_id_since(
                                &self.conn,
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
                                .update(&self.conn, Revision::new(original, user))
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
        .await;

        match result {
            Ok(ok) => {
                tx.commit().await.map_err(|e| {
                    error!(user_id=%username, error=%e, "commit transaction failed");
                    AuthenticationError::InnerError {
                        message: e.to_string(),
                        source: Box::new(e),
                    }
                })?;
                Ok(ok)
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    error!(user_id=%username, error=%e, "rollback transaction failed");
                    AuthenticationError::InnerError {
                        message: e.to_string(),
                        source: Box::new(e),
                    }
                })?;
                Err(e)
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
