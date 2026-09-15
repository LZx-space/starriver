use starriver_identity_domain::{
    password_encoder::PasswordEncoder, password_service::PasswordDomainService,
    user::factory::UserFactory,
};
use starriver_shared_base::{
    authentication::UserIdentifier,
    db::{Connection, Revision, Transaction},
    dto::{PageQuery, PageResult},
    error::RepositoryError,
};
use std::convert::Infallible;
use tracing::{error, info, warn};

use crate::{
    dto::user_dto::{
        req::{ChangeMyPasswordCmd, ResetPasswordCmd, UserRegisterCmd, UserRegisterEmailCmd},
        res::UserDetailDto,
    },
    error::CtxError,
    port::{
        email_verification_service::EmailVerificationService, user_query::UserQuery,
        user_repository::UserRepository,
    },
};

pub struct UserInteractor<Conn, UQ, UR, VCS, PE> {
    conn: Conn,
    user_query: UQ,
    user_repo: UR,
    user_factory: UserFactory<PE>,
    verification_code_service: VCS,
    pwd_service: PasswordDomainService<PE>,
}

impl<Conn, UQ, UR, VCS, PE> UserInteractor<Conn, UQ, UR, VCS, PE>
where
    Conn: Connection,
    UQ: UserQuery<Conn> + Sync,
    UR: UserRepository<Conn> + UserRepository<<Conn as Connection>::Transaction> + Sync,
    VCS: EmailVerificationService + Send + Sync,
    PE: PasswordEncoder + Send + Sync,
{
    /// 新建
    pub fn new(
        conn: Conn,
        user_query: UQ,
        user_repo: UR,
        user_factory: UserFactory<PE>,
        verification_code_service: VCS,
        pwd_service: PasswordDomainService<PE>,
    ) -> Self {
        Self {
            conn,
            user_query,
            user_repo,
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
    pub async fn send_register_email(&self, cmd: UserRegisterEmailCmd) -> Result<(), Infallible> {
        let email = cmd.email.as_str();
        match self.user_query.exists_by_email(&self.conn, email).await {
            Ok(found) => {
                if found {
                    warn!(to=%email, "email already registered, skipping verification");
                    return Ok(());
                }
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

            let original = user.clone();
            self.pwd_service.change_password(
                &mut user,
                cmd.cur_password.as_str(),
                cmd.new_password.as_str(),
            )?;

            self.user_repo
                .update(&tx, Revision::new(original, user))
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
    /// and sends it to the email of the account identified by `user_identifier`.
    ///
    /// If no account matches, it silently returns `Ok(())` to prevent enumeration.
    ///
    /// # Arguments
    /// * `user_identifier` - Either the username or the email of the target account.
    ///
    /// # Errors
    /// Returns `CtxError` if:
    /// - The verification code generation or caching fails.
    /// - The email delivery service returns an error.
    pub async fn send_verification_code(&self, user_identifier: &str) -> Result<(), CtxError> {
        // 把 username / email 统一解析为收件邮箱，两臂产出同一类型以便统一处理
        let resolved = match UserIdentifier::new(user_identifier) {
            UserIdentifier::Email(email) => self
                .user_query
                .exists_by_email(&self.conn, email)
                .await
                .map(|exists| exists.then(|| email.to_owned())),
            UserIdentifier::Username(username) => {
                self.user_query
                    .find_email_by_username(&self.conn, username)
                    .await
            }
        };

        let email = match resolved {
            Ok(Some(email)) => email,
            Ok(None) => {
                warn!(identifier = %user_identifier, "account not found, skip sending code");
                return Ok(());
            }
            Err(e) => {
                error!(identifier = %user_identifier, error = %e, "resolve email by identifier failed");
                return Ok(());
            }
        };

        if let Err(e) = self.verification_code_service.send_code(&email).await {
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
        let user_identifier = &cmd.identifier;
        let email_result = match UserIdentifier::new(user_identifier) {
            UserIdentifier::Email(email) => self
                .user_query
                .exists_by_email(&self.conn, email)
                .await
                .map(|exists| exists.then(|| email.to_owned())),
            UserIdentifier::Username(username) => {
                self.user_query
                    .find_email_by_username(&self.conn, username)
                    .await
            }
        };

        let email = &match email_result {
            Ok(Some(email)) => email,
            Ok(None) => {
                warn!(identifier = %user_identifier, "account not found, skip sending code");
                return Ok(());
            }
            Err(e) => {
                error!(identifier = %user_identifier, error = %e, "resolve email by identifier failed");
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
                            error!(identifier = %user_identifier, error=%e, "commit transaction failed");
                            CtxError::Internal
                        })?;
                        Ok(())
                    }
                    Err(e) => {
                        tx.rollback().await.map_err(|e| {
                            error!(identifier = %user_identifier, error=%e, "rollback transaction failed");
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
