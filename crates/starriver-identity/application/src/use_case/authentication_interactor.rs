use starriver_identity_domain::{
    password_encoder::PasswordEncoder, password_service::PasswordDomainService, user::entity::User,
};
use starriver_shared_base::{
    authentication::authentication_request::{IdentifierPasswordRequest, UserIdentifier},
    authentication::core::error::AuthenticationError,
    db::{Connection, Revision},
    error::RepositoryError,
};
use tracing::{error, info};

use crate::{
    dto::user_dto::{
        input::{SecurityEventCmd, SecurityEventType},
        output::UserDetail,
    },
    port::{security_event_port::SecurityEventPort, user_repository::UserRepository},
};

pub struct AuthenticationInteractor<Conn, UR, SEP, PE> {
    conn: Conn,
    user_repo: UR,
    security_event_port: SEP,
    pwd_service: PasswordDomainService<PE>,
}

impl<Conn, UR, SEP, PE> AuthenticationInteractor<Conn, UR, SEP, PE>
where
    Conn: Connection,
    UR: UserRepository<Conn> + Sync,
    SEP: SecurityEventPort<Conn> + Sync,
    PE: PasswordEncoder + Send + Sync,
{
    /// 新建
    pub fn new(
        conn: Conn,
        user_repo: UR,
        security_event_port: SEP,
        pwd_service: PasswordDomainService<PE>,
    ) -> Self {
        Self {
            conn,
            user_repo,
            security_event_port,
            pwd_service,
        }
    }

    pub async fn authenticate(
        &self,
        req: &IdentifierPasswordRequest,
    ) -> Result<UserDetail, AuthenticationError> {
        let identifier = req.identifier.as_str();
        let password = req.password.as_str();

        // 读用户与密码校验（Argon2）都在事务外完成，避免长时间占用连接
        let user_result = match UserIdentifier::new(identifier) {
            UserIdentifier::Username(username) => {
                self.user_repo.find_by_username(&self.conn, username).await
            }
            UserIdentifier::Email(email) => self.user_repo.find_by_email(&self.conn, email).await,
        };
        let mut user = user_result.map_err(mapping_repo_error())?.ok_or_else(|| {
            info!(identifier = %identifier, "user not found");
            AuthenticationError::IdentifierNotFound
        })?;

        let original = user.clone();
        match self.pwd_service.authenticate(&mut user, password) {
            Ok(_) => {
                // 认证成功：清理失败计数/计数窗口/锁定（若有）。
                // 单条更新，不需要事务。
                if user.has_password_attempts() {
                    user.reset_password_attempts();
                    let user = self
                        .user_repo
                        .update(&self.conn, Revision::new(original, user))
                        .await
                        .map_err(mapping_repo_error())?;

                    self.security_event_port
                        .insert(
                            &self.conn,
                            SecurityEventCmd::new(
                                *user.id(),
                                user.username().to_string(),
                                SecurityEventType::UserUnlocked,
                                "password lock expired",
                            ),
                        )
                        .await;
                    Ok(into_user_detail(user))
                } else {
                    Ok(into_user_detail(user))
                }
            }
            Err(AuthenticationError::BadPassword) => {
                // 失败计数是主业务；审计写入在其后 best-effort 执行，失败不回滚计数。
                let user = self
                    .user_repo
                    .update(&self.conn, Revision::new(original, user))
                    .await
                    .map_err(mapping_repo_error())?;

                let (user_id, username, ..) = user.into_parts();
                self.security_event_port
                    .insert(
                        &self.conn,
                        SecurityEventCmd::new(
                            user_id,
                            username.into_inner(),
                            SecurityEventType::TryLoginWithBadPwd,
                            "bad password",
                        ),
                    )
                    .await;
                Err(AuthenticationError::BadPassword)
            }
            Err(e) => Err(e),
        }
    }
}

fn into_user_detail(user: User) -> UserDetail {
    let (id, username, _, email, ..) = user.into_parts();
    UserDetail {
        id,
        username: username.into_inner(),
        email: email.into_inner(),
    }
}

fn mapping_repo_error() -> impl FnOnce(RepositoryError) -> AuthenticationError {
    |e| {
        error!(error=%e, "query repository error when authentication");
        AuthenticationError::InnerError {
            message: e.to_string(),
        }
    }
}
