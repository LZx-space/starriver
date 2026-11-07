use std::sync::Arc;

use starriver_identity_domain::{
    password_encoder::PasswordEncoder,
    user::{
        entity::User,
        specification::{EmailSpec, PasswordSpec, UsernameSpec},
        value_object::{Email, HashedPassword, RawPassword, Username},
    },
};
use starriver_shared_base::db::Connection;
use tracing::{error, info, warn};

use crate::{
    dto::user_dto::input::UserRegisterCmd,
    error::CtxError,
    port::{
        email_verification_service::EmailVerificationService, user_query::UserQuery,
        user_repository::UserRepository,
    },
};

pub struct RegisterInteractor<Conn, UQ, UR, VCS, PE> {
    conn: Conn,
    user_query: UQ,
    user_repo: UR,
    verification_code_service: VCS,
    email_spec: Arc<EmailSpec>,
    username_spec: Arc<UsernameSpec>,
    password_spec: Arc<PasswordSpec>,
    password_encoder: Arc<PE>,
}

impl<Conn, UQ, UR, VCS, PE> RegisterInteractor<Conn, UQ, UR, VCS, PE>
where
    Conn: Connection,
    UQ: UserQuery<Conn> + Sync,
    UR: UserRepository<Conn> + UserRepository<<Conn as Connection>::Transaction> + Sync,
    VCS: EmailVerificationService + Send + Sync,
    PE: PasswordEncoder + Send + Sync,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        conn: Conn,
        user_query: UQ,
        user_repo: UR,
        verification_code_service: VCS,
        email_spec: Arc<EmailSpec>,
        username_spec: Arc<UsernameSpec>,
        password_spec: Arc<PasswordSpec>,
        password_encoder: Arc<PE>,
    ) -> Self {
        Self {
            conn,
            user_query,
            user_repo,
            verification_code_service,
            email_spec,
            username_spec,
            password_spec,
            password_encoder,
        }
    }

    /// 发送邮箱验证邮件，永远不返回失败以防暴力核验邮箱
    pub async fn send_register_email(&self, email: &str) -> Result<(), CtxError> {
        let email = &Email::new(email, &self.email_spec)?;
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
        let username = Username::new(&cmd.username, &self.username_spec)?;
        let password = RawPassword::new(&cmd.password, &self.password_spec)?;
        let email = &Email::new(&cmd.email, &self.email_spec)?;

        let verification_code = cmd.verification_code.as_str();
        let matches = self
            .verification_code_service
            .check_code(email, verification_code)
            .await
            .inspect_err(|e| info!(email=%email, error=%e, "register user check code failed"))?;
        if !matches {
            return Err(CtxError::InvalidInput(
                "Invalid verification code".to_string(),
            ));
        }

        let hashed_password = HashedPassword::from_raw(&password, self.password_encoder.as_ref())?;
        let user = User::new(username, hashed_password, email.clone());
        self.user_repo
            .insert(&self.conn, user)
            .await
            .inspect_err(|e| error!(error=%e, "repository insert user failed"))?;

        // 业务成功后再消费验证码；消费失败只记日志，用户已经创建成功
        if let Err(e) = self
            .verification_code_service
            .consume_code(email, verification_code)
            .await
        {
            error!(email=%email, error=%e, "consume verification code failed");
        }
        Ok(())
    }
}
