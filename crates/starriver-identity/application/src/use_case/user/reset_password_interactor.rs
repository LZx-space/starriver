use std::sync::Arc;

use starriver_identity_domain::{
    password_encoder::PasswordEncoder,
    password_service::PasswordDomainService,
    user::{
        specification::{EmailSpec, PasswordSpec},
        value_object::{Email, RawPassword},
    },
};
use starriver_shared_base::db::{Connection, Revision};
use time::OffsetDateTime;
use tracing::{error, warn};

use crate::{
    dto::user_dto::input::{ResetPasswordCmd, SecurityEventCmd, SecurityEventKind},
    error::CtxError,
    port::{
        email_verification_service::EmailVerificationService,
        security_event_port::SecurityEventPort, user_query::UserQuery,
        user_repository::UserRepository,
    },
};

pub struct ResetPasswordInteractor<Conn, UQ, UR, SEP, VCS, PE> {
    conn: Conn,
    user_query: UQ,
    user_repo: UR,
    security_event_port: SEP,
    verification_code_service: VCS,
    password_service: PasswordDomainService<PE>,
    password_spec: Arc<PasswordSpec>,
    email_spec: Arc<EmailSpec>,
}

impl<Conn, UQ, UR, SEP, VCS, PE> ResetPasswordInteractor<Conn, UQ, UR, SEP, VCS, PE>
where
    Conn: Connection,
    UQ: UserQuery<Conn> + Sync,
    UR: UserRepository<Conn> + UserRepository<<Conn as Connection>::Transaction> + Sync,
    SEP: SecurityEventPort<Conn> + Sync,
    VCS: EmailVerificationService + Send + Sync,
    PE: PasswordEncoder + Send + Sync,
{
    /// 新建
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        conn: Conn,
        user_query: UQ,
        user_repo: UR,
        security_event_port: SEP,
        verification_code_service: VCS,
        password_service: PasswordDomainService<PE>,
        password_spec: Arc<PasswordSpec>,
        email_spec: Arc<EmailSpec>,
    ) -> Self {
        Self {
            conn,
            user_query,
            user_repo,
            security_event_port,
            verification_code_service,
            password_service,
            password_spec,
            email_spec,
        }
    }

    /// Generates a 6-digit verification code, stores it in the cache with TTL,
    /// and sends it to the email of the account.
    ///
    /// If no account matches, it silently returns `Ok(())` to prevent enumeration.
    ///
    /// # Arguments
    /// * `email` - registered user's email.
    pub async fn send_verification_code(&self, email: &str) -> Result<(), CtxError> {
        let email = &Email::new(email, &self.email_spec)?;
        let email = match self.user_query.exists_by_email(&self.conn, email).await {
            Ok(true) => email,
            Ok(false) => {
                warn!(email = %email, "account not found, skip sending code");
                return Ok(());
            }
            Err(e) => {
                error!(email = %email, error = %e, "find user by email failed");
                return Ok(());
            }
        };

        if let Err(e) = self.verification_code_service.send_code(email).await {
            error!(to = %email, error = %e, "send verification email failed");
        }
        Ok(())
    }

    /// Resets the user's password using a verification code.
    ///
    /// # Arguments
    /// * `cmd` - Contains the target email, verification code, and the new password.
    ///
    /// # Errors
    /// Returns `CtxError` if the code is invalid/expired, or the password update fails.
    pub async fn reset_password_with_verification_code(
        &self,
        cmd: ResetPasswordCmd,
    ) -> Result<(), CtxError> {
        if cmd.new_password.ne(&cmd.new_password_confirm) {
            return Err(CtxError::InvalidInput(
                "New password and confirmation do not match".to_owned(),
            ));
        }
        let raw_password = &RawPassword::new(&cmd.new_password, &self.password_spec)?;
        let email = &Email::new(&cmd.email, &self.email_spec)?;
        // 先验码后查库：验证码无效时零 DB 访问，且“账号不存在”与“验证码错误”响应完全一致，防枚举
        let matches = self
            .verification_code_service
            .check_code(email, &cmd.verification_code)
            .await
            .map_err(|e| CtxError::internal("reset password", e))?;
        if !matches {
            return Err(CtxError::InvalidInput(
                "Invalid verification code".to_owned(),
            ));
        }

        let Some(mut user) = self.user_repo.find_by_email(&self.conn, email).await? else {
            // 验证码有效却找不到账号：发码后账号被删除/更换邮箱
            warn!(email = %email, "account not found after code validation");
            return Err(CtxError::InvalidInput(
                "Invalid verification code".to_owned(),
            ));
        };

        let origin = user.clone();
        self.password_service
            .reset_password(&mut user, raw_password)?;

        let updated = self
            .user_repo
            .update(&self.conn, Revision::new(origin, user))
            .await;

        match updated {
            Ok(user) => {
                self.security_event_port
                    .insert_many(
                        &self.conn,
                        vec![SecurityEventCmd::new(
                            &user,
                            SecurityEventKind::PasswordReset,
                            OffsetDateTime::now_utc(),
                            None,
                        )],
                    )
                    .await;

                // 业务提交成功后再消费验证码；消费失败只记日志
                if let Err(e) = self
                    .verification_code_service
                    .consume_code(email, &cmd.verification_code)
                    .await
                {
                    error!(email=%email, error=%e, "consume verification code failed");
                }
                Ok(())
            }
            Err(e) => Err(CtxError::from(e)),
        }
    }
}
