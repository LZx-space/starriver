use starriver_identity_domain::{
    password_encoder::PasswordEncoder, password_service::PasswordDomainService,
};
use starriver_shared_base::{
    authentication::authentication_request::{IdentifierPasswordRequest, UserIdentifier},
    authentication::core::error::AuthenticationError,
    db::{Connection, Revision, Transaction},
    error::RepositoryError,
};
use tracing::{error, info};

use crate::{
    dto::user_dto::{
        req::{SecurityEventCmd, SecurityEventType},
        res::UserDetail,
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
    UR: UserRepository<<Conn as Connection>::Transaction> + Sync,
    SEP: SecurityEventPort<<Conn as Connection>::Transaction> + Sync,
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

        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            AuthenticationError::InnerError {
                message: e.to_string(),
            }
        })?;

        match async {
            let user_result = match UserIdentifier::new(identifier) {
                UserIdentifier::Username(username) => {
                    self.user_repo.find_by_username(&tx, username).await
                }
                UserIdentifier::Email(email) => self.user_repo.find_by_email(&tx, email).await,
            };
            let mut user = user_result.map_err(mapping_repo_error())?.ok_or_else(|| {
                info!(identifier = %identifier, "user not found");
                AuthenticationError::IdentifierNotFound
            })?;

            match self.pwd_service.authenticate(&mut user, password) {
                Ok(_) => Ok(user),
                Err(AuthenticationError::BadPassword) => {
                    let original = user.clone();
                    let user = self
                        .user_repo
                        .update(&tx, Revision::new(original, user))
                        .await
                        .map_err(mapping_repo_error())?;
                    let user_id = user.dissolve().0;
                    self.security_event_port
                        .insert(
                            &tx,
                            SecurityEventCmd {
                                user_id,
                                event_type: SecurityEventType::TryLoginWithBadPwd,
                                payload: "bad password".to_string(),
                            },
                        )
                        .await
                        .map_err(mapping_repo_error())?;
                    Err(AuthenticationError::BadPassword)
                }
                Err(e) => Err(e),
            }
        }
        .await
        {
            Ok(user) => {
                tx.commit()
                    .await
                    .map_err(|e| AuthenticationError::InnerError {
                        message: e.to_string(),
                    })?;
                let fields = user.dissolve();
                Ok(UserDetail {
                    id: fields.0,
                    username: fields.1.to_string(),
                    email: fields.3.to_string(),
                })
            }
            Err(AuthenticationError::BadPassword) => {
                tx.commit()
                    .await
                    .map_err(|e| AuthenticationError::InnerError {
                        message: e.to_string(),
                    })?;
                Err(AuthenticationError::BadPassword)
            }
            Err(e) => {
                tx.rollback()
                    .await
                    .map_err(|e| AuthenticationError::InnerError {
                        message: e.to_string(),
                    })?;
                Err(e)
            }
        }
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
