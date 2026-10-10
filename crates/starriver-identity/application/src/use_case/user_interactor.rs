use starriver_identity_domain::{
    password_encoder::PasswordEncoder, password_service::PasswordDomainService,
    user::factory::UserFactory,
};
use starriver_shared_base::{
    db::{Connection, Revision, Transaction},
    dto::{PageQuery, PageResult},
    error::RepositoryError,
};
use std::convert::Infallible;
use tracing::{error, info, warn};

use crate::{
    dto::user_dto::{
        req::{
            ChangeMyPasswordCmd, ResetPasswordCmd, SecurityEventCmd, SecurityEventType,
            UserRegisterCmd,
        },
        res::UserDetailDto,
    },
    error::CtxError,
    port::{
        email_verification_service::EmailVerificationService,
        security_event_port::SecurityEventPort, user_query::UserQuery,
        user_repository::UserRepository,
    },
};

pub struct UserInteractor<Conn, UQ, UR, SEP, VCS, PE> {
    conn: Conn,
    user_query: UQ,
    user_repo: UR,
    security_event_port: SEP,
    user_factory: UserFactory<PE>,
    verification_code_service: VCS,
    pwd_service: PasswordDomainService<PE>,
}

impl<Conn, UQ, UR, SEP, VCS, PE> UserInteractor<Conn, UQ, UR, SEP, VCS, PE>
where
    Conn: Connection,
    UQ: UserQuery<Conn> + Sync,
    UR: UserRepository<Conn> + UserRepository<<Conn as Connection>::Transaction> + Sync,
    SEP: SecurityEventPort<<Conn as Connection>::Transaction> + Sync,
    VCS: EmailVerificationService + Send + Sync,
    PE: PasswordEncoder + Send + Sync,
{
    /// 新建
    pub fn new(
        conn: Conn,
        user_query: UQ,
        user_repo: UR,
        security_event_port: SEP,
        user_factory: UserFactory<PE>,
        verification_code_service: VCS,
        pwd_service: PasswordDomainService<PE>,
    ) -> Self {
        Self {
            conn,
            user_query,
            user_repo,
            security_event_port,
            verification_code_service,
            user_factory,
            pwd_service,
        }
    }

    pub async fn paginate(&self, q: PageQuery) -> Result<PageResult<UserDetailDto>, CtxError> {
        self.user_query
            .paginate(&self.conn, q)
            .await
            .map_err(|e| CtxError::internal("paginate users failed", e))
    }

    ///// register user ///////////////////////////////////////////////////////////////////////

    /// 发送邮箱验证邮件，永远不返回失败以防暴力核验邮箱
    pub async fn send_register_email(&self, email: &str) -> Result<(), Infallible> {
        match self.user_query.exists_by_email(&self.conn, email).await {
            Ok(true) => {
                warn!(to=%email, "email already registered, skipping verification");
                Ok(())
            }
            Ok(false) => {
                if let Err(e) = self.verification_code_service.send_code(email).await {
                    error!(to=%email, error=%e, "send verification email failed");
                }
                Ok(())
            }
            Err(e) => {
                error!(to=%email, error=%e, "find user by email failed");
                Ok(())
            }
        }
    }

    pub async fn register_user(&self, cmd: UserRegisterCmd) -> Result<(), CtxError> {
        let verification_code = cmd.verification_code.as_str();
        let email = cmd.email.as_str();
        let matches = self
            .verification_code_service
            .validate_code(email, verification_code)
            .await
            .inspect_err(|e| info!(email=%email, error=%e, "register user validate code failed"))?;
        if !matches {
            return Err(CtxError::InvalidInput("invalid email code".to_string()));
        }
        let user = self
            .user_factory
            .create_user(cmd.username.as_str(), cmd.password.as_str(), email)
            .inspect_err(|e| info!(email=%email, error=%e, "register user create user failed"))?;

        self.user_repo
            .insert(&self.conn, user)
            .await
            .inspect_err(|e| error!(email=%email, error=%e, "repository insert user failed"))?;
        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////

    /// change password for authenticated user
    pub async fn change_my_password(
        &self,
        username: &str,
        cmd: ChangeMyPasswordCmd,
    ) -> Result<(), CtxError> {
        if cmd.new_password != cmd.new_password_confirm {
            return Err(CtxError::InvalidInput(
                "new password and confirm password do not match".to_string(),
            ));
        }
        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;

        let result: Result<(), CtxError> = async {
            let user_opt = self.user_repo.find_by_username(&tx, username).await?;
            let mut user =
                user_opt.ok_or(RepositoryError::NotFound("user not found".to_string()))?;

            let user_id = *user.id();
            let original = user.clone();
            self.pwd_service.change_password(
                &mut user,
                cmd.cur_password.as_str(),
                cmd.new_password.as_str(),
            )?;

            self.user_repo
                .update(&tx, Revision::new(original, user))
                .await?;
            self.security_event_port
                .insert(
                    &tx,
                    SecurityEventCmd {
                        user_id,
                        event_type: SecurityEventType::PasswordChanged,
                        payload: "password changed".to_string(),
                    },
                )
                .await?;
            Ok(())
        }
        .await;

        match result {
            Ok(_) => {
                tx.commit().await.map_err(|e| {
                    error!(error=%e, "commit transaction failed");
                    CtxError::Internal
                })?;
                Ok(())
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    error!(username=%username, error=%e, "rollback transaction failed");
                    CtxError::Internal
                })?;
                Err(e)
            }
        }
    }

    /// Generates a 6-digit verification code, stores it in the cache with TTL,
    /// and sends it to the email of the account.
    ///
    /// If no account matches, it silently returns `Ok(())` to prevent enumeration.
    ///
    /// # Arguments
    /// * `email` - registered user's email.
    pub async fn send_verification_code(&self, email: &str) -> Result<(), Infallible> {
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
                "new password does not match".to_owned(),
            ));
        }
        let email = &cmd.email;
        // 先验码后查库：验证码无效时零 DB 访问，且“账号不存在”与“验证码错误”响应完全一致，防枚举
        let matches = self
            .verification_code_service
            .validate_code(email, &cmd.verification_code)
            .await
            .map_err(|e| CtxError::internal("reset password", e))?;
        if !matches {
            return Err(CtxError::InvalidInput(
                "verification code invalid".to_owned(),
            ));
        }

        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;
        let result = async {
            let Some(mut user) = self.user_repo.find_by_email(&tx, email).await? else {
                // 验证码有效却找不到账号：发码后账号被删除/更换邮箱
                warn!(email = %email, "account not found after code validation");
                return Err(CtxError::InvalidInput(
                    "verification code invalid".to_owned(),
                ));
            };
            let user_id = *user.id();
            let origin = user.clone();
            self.pwd_service
                .reset_password(&mut user, &cmd.new_password)?;
            self.user_repo
                .update(&tx, Revision::new(origin, user))
                .await?;
            self.security_event_port
                .insert(
                    &tx,
                    SecurityEventCmd {
                        user_id,
                        event_type: SecurityEventType::PasswordChanged,
                        payload: "password reset with verification code".to_string(),
                    },
                )
                .await?;
            Ok(())
        }
        .await;

        match result {
            Ok(_) => {
                tx.commit().await.map_err(|e| {
                    error!(email = %email, error=%e, "commit transaction failed");
                    CtxError::Internal
                })?;
                Ok(())
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| {
                    error!(email = %email, error=%e, "rollback transaction failed");
                    CtxError::Internal
                })?;
                Err(e)
            }
        }
    }

    /// Generates a 6-digit verification code, stores it in the cache with TTL,
    /// and sends it to the email of the account identified by `user_identifier`.
    ///
    /// If no account matches, it silently returns `Ok(())` to prevent enumeration.
    ///
    /// # Arguments
    /// * `email` - registed user's email.
    ///
    /// # Errors
    /// Returns `CtxError` if:
    /// - The verification code generation or caching fails.
    /// - The email delivery service returns an error.
    pub async fn send_verification_code(&self, email: &str) -> Result<(), CtxError> {
        let email = match self.user_query.exists_by_email(&self.conn, email).await {
            Ok(true) => email,
            Ok(false) => {
                warn!(email = %email, "account not found, skip sending code");
                return Ok(());
            }
            Err(e) => {
                error!(email = %email, error = %e, "resolve email by identifier failed");
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
    /// * `cmd` - Contains the target username/email, verification code, and the new password.
    ///
    /// # Errors
    /// Returns `CtxError` if the username is invalid, the code is incorrect/expired,
    /// or the password update fails.
    pub async fn reset_password_with_verification_code(
        &self,
        cmd: ResetPasswordCmd,
    ) -> Result<(), CtxError> {
        if cmd.new_password.ne(&cmd.new_password_confirm) {
            return Err(CtxError::InvalidInput(
                "new password does not match".to_owned(),
            ));
        }
        let email = &cmd.email;
        let email = match self.user_query.exists_by_email(&self.conn, email).await {
            Ok(true) => email,
            Ok(false) => {
                warn!(email = %email, "account not found, skip sending code");
                return Ok(());
            }
            Err(e) => {
                error!(email = %email, error = %e, "resolve email by identifier failed");
                return Ok(());
            }
        };
        match self
            .verification_code_service
            .validate_code(email, &cmd.verification_code)
            .await
        {
            Ok(true) => {
                let tx = self.conn.begin().await.map_err(|e| {
                    error!(error = %e, "begin transaction failed");
                    CtxError::Internal
                })?;
                let result = async {
                    let Some(mut user) = self.user_repo.find_by_email(&self.conn, email).await?
                    else {
                        return Err(CtxError::NotFound(email.to_owned()));
                    };
                    let origin = user.clone();
                    self.pwd_service
                        .reset_password(&mut user, &cmd.new_password)?;
                    self.user_repo
                        .update(&self.conn, Revision::new(origin, user))
                        .await?;
                    Ok(())
                }
                .await;

                match result {
                    Ok(_) => {
                        tx.commit().await.map_err(|e| {
                            error!(email = %email, error=%e, "commit transaction failed");
                            CtxError::Internal
                        })?;
                        Ok(())
                    }
                    Err(e) => {
                        tx.rollback().await.map_err(|e| {
                            error!(email = %email, error=%e, "rollback transaction failed");
                            CtxError::Internal
                        })?;
                        Err(e)
                    }
                }
            }
            Ok(false) => Err(CtxError::InvalidInput(
                "verification code invalid".to_owned(),
            )),
            Err(e) => Err(CtxError::internal("reset password", e)),
        }
    }
}
