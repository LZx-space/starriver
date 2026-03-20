use crate::db::user_do::ActiveModel;
use crate::db::user_do::Entity;
use crate::db::user_do::Model;
use crate::db::user_do::ModelEx;
use crate::db::user_do::UserStateDo;
use crate::db::user_security_event_do;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::HasOneModel;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use starriver_domain::user::entity::SecurityEvent;
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::value_object::{Password, Username};
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::error::Cause;
use time::Duration;
use time::OffsetDateTime;

pub struct DefaultUserRepository {
    pub conn: &'static DatabaseConnection,
}

impl UserRepository for DefaultUserRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        find_by_username(self.conn, username).await
    }

    async fn insert(&self, user: User) -> Result<User, ApiError> {
        let username = user.username.as_str();
        let found = self.find_by_username(username).await?;
        if let Some(user) = found {
            return Err(ApiError::new(
                Cause::ClientBadRequest,
                format!("username[{}] already exists", user.username.as_str()),
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

    async fn update(&self, mut user: User) -> Result<User, ApiError> {
        self.conn
            .transaction::<_, User, ApiError>(|tx| {
                Box::pin(async move {
                    match find_by_username(tx, user.username.as_str()).await? {
                        Some(found) => {
                            let mut model = ActiveModel::builder()
                                .set_id(found.id)
                                .set_update_at(Some(OffsetDateTime::now_utc()));
                            if found.username != user.username {
                                model = model.set_username(user.username.as_str().to_string());
                            }
                            if found.password != user.password {
                                model = model.set_password(
                                    user.password.hashed_password_string().to_string(),
                                );
                            }
                            if found.state != user.state {
                                let state: UserStateDo = user.state.into();
                                model = model.set_state(state);
                            }

                            if found.security_events != user.security_events
                                && let Some(event) = user.security_events.pop()
                            {
                                let event_model = user_security_event_do::ActiveModelEx {
                                    id: Set(event.id),
                                    user_id: Set(event.user_id),
                                    event_type: Set(event.event_type.into()),
                                    message: Set(event.message),
                                    create_at: Set(event.created_at),
                                    update_at: NotSet,
                                    user: HasOneModel::NotSet,
                                };
                                event_model.insert(tx).await?;
                            }
                            model.update(tx).await.map(model_ex_to_entity)?
                        }
                        None => Err(ApiError::new(Cause::ClientBadRequest, "User not found")),
                    }
                })
            })
            .await
            .map_err(ApiError::from)
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

/// 为 事务&非事务链接 共享
async fn find_by_username(
    conn: &impl sea_orm::ConnectionTrait,
    username: &str,
) -> Result<Option<User>, ApiError> {
    let user = Entity::load()
        .filter_by_username(username)
        .one(conn)
        .await?
        .map(model_ex_to_entity)
        .transpose()?;
    if let Some(mut user) = user {
        let mut events: Vec<SecurityEvent> = user_security_event_do::Entity::find()
            .filter(user_security_event_do::Column::UserId.eq(user.id))
            .filter(
                user_security_event_do::Column::CreateAt
                    .gt(OffsetDateTime::now_utc().saturating_sub(Duration::minutes(30))), // 最近30分钟内的事件，注意当前此处没有与PasswordSpecification同步
            )
            .order_by_desc(user_security_event_do::Column::CreateAt) // 按时间倒序取最新
            .all(conn)
            .await?
            .iter()
            .map(|e| SecurityEvent {
                id: e.id,
                user_id: e.user_id,
                event_type: e.event_type.clone().into(),
                message: e.message.to_owned(),
                created_at: e.create_at,
            })
            .collect();
        user.security_events.append(&mut events);
        return Ok(Some(user));
    }
    Ok(user)
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
        security_events: vec![],
    })
}

#[inline]
fn model_ex_to_entity(m: ModelEx) -> Result<User, ApiError> {
    let username = Username::new(m.username.as_str())?;
    let password = Password::restore_by_hashed_pwd(m.password.as_str(), OffsetDateTime::now_utc())?;
    Ok(User {
        id: m.id,
        username,
        password,
        state: m.state.into(),
        created_at: m.create_at,
        security_events: vec![],
    })
}
