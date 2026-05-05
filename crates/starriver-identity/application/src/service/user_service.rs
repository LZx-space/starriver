use std::convert::Infallible;

use starriver_identity_domain::{
    aggregate::{
        authentication_service::AuthenticationService, user_factory::UserFactory,
        user_policy::BadPassswordPolicy, user_repository::UserRepository,
    },
    common::{error::RepositoryError, traits::PasswordEncoder},
};
use tracing::{error, info, warn};

use crate::{
    common::{error::AppError, regex_patterns::Patterns},
    dto::user_dto::{
        req::{EmailVerifyCmd, UserCmd, UsernamePasswordCredentials},
        res::UserDetail,
    },
    port_out::{user_query_port::UserQueryPort, verification_code_port::VerificationCodePort},
};

pub struct UserApplicationService<
    UQP: UserQueryPort,
    REPO: UserRepository,
    VCP: VerificationCodePort,
    PE: PasswordEncoder,
> {
    query: UQP,
    repo: REPO,
    verification_code_port: VCP,
    factory: UserFactory<PE>,
    auth_service: AuthenticationService<PE>,
}

impl<UQP, REPO, VCP, PE> UserApplicationService<UQP, REPO, VCP, PE>
where
    UQP: UserQueryPort,
    REPO: UserRepository,
    VCP: VerificationCodePort,
    PE: PasswordEncoder + Clone,
{
    /// 新建
    pub fn new(
        user_query_port: UQP,
        user_repo: REPO,
        verification_code_port: VCP,
        patterns: Patterns,
        bad_password_policy: BadPassswordPolicy,
        password_encoder: PE,
    ) -> Self {
        let factory = UserFactory::new(
            patterns.email,
            patterns.username,
            patterns.password,
            password_encoder.clone(),
        );
        let auth_service = AuthenticationService::new(bad_password_policy, password_encoder);

        Self {
            query: user_query_port,
            repo: user_repo,
            verification_code_port,
            factory,
            auth_service,
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
                if let Err(e) = self.verification_code_port.send_code(email).await {
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

    pub async fn register_user(&self, cmd: UserCmd) -> Result<(), AppError> {
        let email_code = cmd.email_code.as_str();
        let email = cmd.email.as_str();
        self.verification_code_port
            .validate_code(email, email_code)
            .await
            .inspect_err(|e| info!(email=%email, error=%e, "rigister user validate code failed"))?;
        let user = self
            .factory
            .create_user(cmd.username.as_str(), cmd.password.as_str(), email)
            .inspect_err(|e| info!(email=%email, error=%e, "rigister user create user failed"))?;
        self.repo
            .insert(user)
            .await
            .map(|_| ())
            .map_err(AppError::from)
            .inspect_err(|e| error!(email=%email, error=%e, "repository insert user failed"))
    }

    pub async fn authenticate(
        &self,
        credentials: &UsernamePasswordCredentials,
    ) -> Result<UserDetail, AppError> {
        let username = credentials.username.as_str();
        let password = credentials.password.as_str();
        let user = self
            .repo
            .find_by_username(username)
            .await
            .map_err(AppError::from)?;

        let Some(user) = user else {
            info!(username = %username, "user not found");
            return Err(AppError::from(RepositoryError::NotFound(
                "user not found".to_string(),
            )));
        };

        self.auth_service.authenticate(&user, password)?;

        let fields = user.dissolve();
        Ok(UserDetail {
            id: fields.0,
            username: fields.1.to_string(),
            email: fields.3.to_string(),
        })
    }
}
