use starriver_identity_domain::{
    password_encoder::PasswordEncoder, password_service::PasswordDomainService,
};
use starriver_shared_base::{
    authentication::UsernamePasswordCredentials,
    db::{Connection, Revision, Transaction},
    error::RepositoryError,
    middleware::authentication::core::error::AuthenticationError,
};
use tracing::{error, info};

use crate::{
    dto::user_dto::{
        req::{SecurityEventCmd, SecurityEventType},
        res::UserDetail,
    },
    port::{security_event_port::SecurityEventPort, user_repository::UserRepository},
};

pub struct AuthenticationInteractor<Conn, UR, SER, PE> {
    conn: Conn,
    user_repo: UR,
    security_event_recorder: SER,
    pwd_service: PasswordDomainService<PE>,
}

impl<Conn, UR, SER, PE> AuthenticationInteractor<Conn, UR, SER, PE>
where
    Conn: Connection,
    UR: UserRepository<<Conn as Connection>::Transaction> + Sync,
    SER: SecurityEventPort<<Conn as Connection>::Transaction> + Sync,
    PE: PasswordEncoder + Send + Sync,
{
    /// 新建
    pub fn new(
        conn: Conn,
        user_repo: UR,
        security_event_recorder: SER,
        pwd_service: PasswordDomainService<PE>,
    ) -> Self {
        Self {
            conn,
            user_repo,
            security_event_recorder,
            pwd_service,
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
            }
        })?;

        match async {
            let mut user = self
                .user_repo
                .find_by_username(&tx, username)
                .await
                .map_err(mapping_repo_error())?
                .ok_or_else(|| {
                    info!(username = %username, "user not found");
                    AuthenticationError::UsernameNotFound
                })?;

            let result = self.pwd_service.authenticate(&mut user, password);

            match result {
                Ok(_) => Ok(user),
                Err(AuthenticationError::BadPassword) => {
                    let original = user.clone();
                    let user = self
                        .user_repo
                        .update(&tx, Revision::new(original, user))
                        .await
                        .map_err(mapping_repo_error())?;
                    let user_id = user.dissolve().0;
                    self.security_event_recorder
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
        error!(error=%e, "handle bad password event failed");
        AuthenticationError::InnerError {
            message: e.to_string(),
        }
    }
}
