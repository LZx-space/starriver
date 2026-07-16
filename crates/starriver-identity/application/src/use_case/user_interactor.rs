use starriver_identity_domain::{
    password_encoder::PasswordEncoder, password_service::PasswordDomainService,
    user::factory::UserFactory,
};
use starriver_shared_base::{
    db::{Connection, Revision, Transaction},
    dto::{PageQuery, PageResult},
};
use std::convert::Infallible;
use tracing::{error, info, warn};

use crate::{
    dto::user_dto::{
        req::{
            ChangePasswordCmd, UserActiveCmd, UserActiveEmailCmd, UserRegisterCmd,
            UserRegisterEmailCmd,
        },
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
        self.user_query.paginate(&self.conn, q).await.map_err(|e| {
            error!(error=%e, "paginate users failed");
            CtxError::Internal
        })
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

    ///// activate user by self ///////////////////////////////////////////////////////////////////////

    /// 发送用户激活邮件，永远不返回失败以防暴力核验邮箱
    pub async fn send_activation_email(&self, cmd: UserActiveEmailCmd) -> Result<(), Infallible> {
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

    ///// change_password by self ///////////////////////////////////////////////////////////////////////

    pub async fn change_password(
        &self,
        username: &str,
        cmd: ChangePasswordCmd,
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

        let mut user = self
            .user_repo
            .find_by_username(&tx, username)
            .await?
            .ok_or(CtxError::NotFound("user not found".to_string()))?;

        let original = user.clone();

        // todo anti timing attack
        self.pwd_service.change_password(
            &mut user,
            cmd.cur_password.as_str(),
            cmd.new_password.as_str(),
        )?;

        match self
            .user_repo
            .update(&tx, Revision::new(original, user))
            .await
        {
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
                error!(username=%username, error=%e, "update user failed");
                Err(CtxError::Internal)
            }
        }
    }
}
