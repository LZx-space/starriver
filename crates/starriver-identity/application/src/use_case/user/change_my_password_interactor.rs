use std::sync::Arc;

use starriver_identity_domain::{
    password_encoder::PasswordEncoder,
    password_service::PasswordDomainService,
    user::{
        specification::{PasswordSpec, UsernameSpec},
        value_object::{RawPassword, Username},
    },
};
use starriver_shared_base::{
    db::{Connection, Revision},
    error::RepositoryError,
};
use time::OffsetDateTime;

use crate::{
    dto::user_dto::input::{ChangeMyPasswordCmd, SecurityEventCmd, SecurityEventKind},
    error::CtxError,
    port::{security_event_port::SecurityEventPort, user_repository::UserRepository},
};

pub struct ChangeMyPasswordInteractor<Conn, UR, SEP, PE> {
    conn: Conn,
    user_repo: UR,
    security_event_port: SEP,
    password_service: PasswordDomainService<PE>,
    username_spec: Arc<UsernameSpec>,
    password_spec: Arc<PasswordSpec>,
}

impl<Conn, UR, SEP, PE> ChangeMyPasswordInteractor<Conn, UR, SEP, PE>
where
    Conn: Connection,
    UR: UserRepository<Conn> + UserRepository<<Conn as Connection>::Transaction> + Sync,
    SEP: SecurityEventPort<Conn> + Sync,
    PE: PasswordEncoder + Send + Sync,
{
    /// 新建
    pub fn new(
        conn: Conn,
        user_repo: UR,
        security_event_port: SEP,
        password_service: PasswordDomainService<PE>,
        username_spec: Arc<UsernameSpec>,
        password_spec: Arc<PasswordSpec>,
    ) -> Self {
        Self {
            conn,
            user_repo,
            security_event_port,
            password_service,
            username_spec,
            password_spec,
        }
    }

    /// change password for authenticated user
    pub async fn change_my_password(
        &self,
        username: &str,
        cmd: ChangeMyPasswordCmd,
    ) -> Result<(), CtxError> {
        if cmd.new_password != cmd.new_password_confirm {
            return Err(CtxError::InvalidInput(
                "New password and confirmation do not match".to_string(),
            ));
        }
        let username = &Username::new(username, &self.username_spec)?;
        let new_password = &RawPassword::new(&cmd.new_password, self.password_spec.as_ref())?;

        let mut user = self
            .user_repo
            .find_by_username(&self.conn, username)
            .await?
            .ok_or(RepositoryError::NotFound("User not found".to_string()))?;

        let original = user.clone();
        let changed = self.password_service.change_password(
            &mut user,
            cmd.cur_password.as_str(),
            new_password,
        );
        let user = self
            .user_repo
            .update(&self.conn, Revision::new(original, user))
            .await?;
        match changed {
            Ok(_) => {
                self.security_event_port
                    .insert_many(
                        &self.conn,
                        vec![SecurityEventCmd::new(
                            &user,
                            SecurityEventKind::PasswordChanged,
                            OffsetDateTime::now_utc(),
                            None,
                        )],
                    )
                    .await;
                Ok(())
            }
            Err(e) => Err(CtxError::from(e)),
        }
    }
}
