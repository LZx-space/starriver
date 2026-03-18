use crate::db::user_do::ActiveModel;
use crate::db::user_do::Entity;
use crate::db::user_do::Model;
use crate::db::user_do::ModelEx;
use crate::db::user_do::UserStateDo;
use crate::db::user_security_event_do;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::HasOneModel;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use starriver_domain::user::entity::SecurityEvent;
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::state_object::AuthByPwdState;
use starriver_domain::user::value_object::{Password, Username};
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::error::Cause;
use time::OffsetDateTime;

pub struct DefaultUserRepository {
    pub conn: &'static DatabaseConnection,
}

impl UserRepository for DefaultUserRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        Entity::load()
            .filter_by_username(username)
            .with(user_security_event_do::Entity)
            .one(self.conn)
            .await?
            .map(model_ex_to_entity)
            .transpose()
    }

    async fn insert(&self, user: User) -> Result<User, ApiError> {
        let username = user.username.as_str();
        let found = self.find_by_username(username).await?;
        if found.is_some() {
            return Err(ApiError::new(
                Cause::ClientBadRequest,
                "username already exists",
            ));
        }
        ActiveModel {
            id: Set(user.id),
            username: Set(username.to_string()),
            password: Set(user.password.hashed_password_string().to_string()),
            state: Set(crate::db::user_do::UserStateDo::Inactive),
            create_at: Set(OffsetDateTime::now_utc()),
            update_at: NotSet,
        }
        .insert(self.conn)
        .await
        .map(model_to_entity)?
    }

    async fn delete(&self, user_id: uuid::Uuid) -> Result<bool, ApiError> {
        let result = Entity::delete_by_id(user_id)
            .exec(self.conn)
            .await?
            .rows_affected
            > 0;
        Ok(result)
    }

    async fn update_auth_pwd_state(&self, state: AuthByPwdState) -> Result<User, ApiError> {
        let mut model = ActiveModel::builder().set_id(state.user_id);
        if state.locked {
            model = model.set_state(UserStateDo::Locked);
        }
        self.conn
            .transaction::<_, User, ApiError>(|tx| {
                Box::pin(async {
                    if let Some(event) = state.bad_pwd_event {
                        let event_model = user_security_event_do::ActiveModelEx {
                            id: Set(event.id),
                            user_id: Set(event.user_id),
                            event_type: Set(event.event_type.into()),
                            message: Set(event.message),
                            create_at: Set(event.created_at),
                            update_at: NotSet,
                            author: HasOneModel::NotSet,
                        };
                        event_model.insert(tx).await?;
                    }
                    model.update(tx).await.map(model_ex_to_entity)?
                })
            })
            .await
            .map_err(ApiError::from)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[inline]
fn model_to_entity(m: Model) -> Result<User, ApiError> {
    let username = Username::new(m.username.as_str())?;
    let password = Password::restore_by_hashed_pwd(m.password.as_str(), OffsetDateTime::now_utc())?;
    Ok(User {
        id: m.id,
        username,
        password,
        state: m.state.into(),
        created_at: m.create_at,
        login_events: vec![],
    })
}

#[inline]
fn model_ex_to_entity(m: ModelEx) -> Result<User, ApiError> {
    let username = Username::new(m.username.as_str())?;
    let password = Password::restore_by_hashed_pwd(m.password.as_str(), OffsetDateTime::now_utc())?;
    let login_events: Vec<SecurityEvent> = m
        .security_events
        .into_iter()
        .map(|e| SecurityEvent {
            id: e.id,
            user_id: e.user_id,
            event_type: e.event_type.into(),
            message: e.message,
            created_at: e.create_at,
        })
        .collect();
    Ok(User {
        id: m.id,
        username,
        password,
        state: m.state.into(),
        created_at: m.create_at,
        login_events,
    })
}
