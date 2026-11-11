use std::sync::Arc;

use starriver_identity_domain::{
    password_encoder::PasswordEncoder,
    user::{
        entity::User,
        policy::BadPasswordPolicy,
        specification::{EmailSpec, UsernameSpec},
        value_object::{Email, Username},
    },
};
use starriver_shared_base::{
    authentication::authentication_request::{IdentifierPasswordRequest, UserIdentifier},
    authentication::core::error::AuthenticationError,
    db::{Connection, Revision},
    error::RepositoryError,
};
use tracing::{error, info};

use crate::{
    dto::user_dto::output::UserDetail,
    port::{security_event_port::SecurityEventPort, user_repository::UserRepository},
};

pub struct AuthenticationInteractor<Conn, UR, SEP, PE> {
    conn: Conn,
    user_repo: UR,
    security_event_port: SEP,
    password_policy: Arc<BadPasswordPolicy>,
    password_encoder: Arc<PE>,
    email_spec: Arc<EmailSpec>,
    username_spec: Arc<UsernameSpec>,
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
        password_policy: Arc<BadPasswordPolicy>,
        password_encoder: Arc<PE>,
        email_spec: Arc<EmailSpec>,
        username_spec: Arc<UsernameSpec>,
    ) -> Self {
        Self {
            conn,
            user_repo,
            security_event_port,
            password_policy,
            password_encoder,
            email_spec,
            username_spec,
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
                let username = &Username::new(username, &self.username_spec)
                    .map_err(|_| AuthenticationError::InvalidUsernameFormat)?;
                self.user_repo.find_by_username(&self.conn, username).await
            }
            UserIdentifier::Email(email) => {
                let email = &Email::new(email, &self.email_spec)
                    .map_err(|_| AuthenticationError::InvalidEmailFormat)?;
                self.user_repo.find_by_email(&self.conn, email).await
            }
        };
        let mut user = user_result.map_err(mapping_repo_error())?.ok_or_else(|| {
            info!(identifier = %identifier, "user not found");
            AuthenticationError::IdentifierNotFound
        })?;

        let original = user.clone();
        let auth_result = user.authenticate(
            password,
            self.password_policy.as_ref(),
            self.password_encoder.as_ref(),
        );
        let events = user.take_events();
        let user = self
            .user_repo
            .update(&self.conn, Revision::new(original, user))
            .await
            .map_err(mapping_repo_error())?;
        if !events.is_empty() {
            self.security_event_port
                .insert_many(&self.conn, events)
                .await;
        }

        match auth_result {
            Ok(_) => Ok(into_user_detail(user)),
            Err(e) => Err(e.into()),
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
