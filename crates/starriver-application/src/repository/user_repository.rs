use crate::db::user_do::ActiveModel;
use crate::db::user_do::Column;
use crate::db::user_do::Entity;
use crate::db::user_do::Model;
use crate::db::user_do::UserStateDo;
use crate::db::user_security_event_do;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveValue::Unchanged;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use starriver_domain::user::entity::SecurityEvent;
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::value_object::Email;
use starriver_domain::user::value_object::{Password, Username};
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::error::Cause;
use starriver_infrastructure::util::db::TransactionalConn;
use time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct DefaultUserRepository<'a, T> {
    conn: &'a T,
}

impl<'a, T> DefaultUserRepository<'a, T>
where
    T: TransactionalConn,
{
    pub fn new(conn: &'a T) -> DefaultUserRepository<'a, T> {
        Self { conn }
    }
}

impl<'a, T> UserRepository for DefaultUserRepository<'a, T>
where
    T: TransactionalConn,
{
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        find_by_username(self.conn, username).await
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, ApiError> {
        let user = Entity::find_by_id(user_id)
            .one(self.conn)
            .await?
            .map(model_to_entity)
            .transpose()?;
        Ok(user)
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
            email: Set(user.email.to_string()),
            state: Set(UserStateDo::Active),
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
        match find_by_username(self.conn, user.username.as_str()).await? {
            Some(found) => {
                let mut username = Unchanged(found.username.as_str().to_string());
                username.set_if_not_equals(user.username.as_str().to_string());

                let mut password = Unchanged(found.password.hashed_password_string().to_string());
                password.set_if_not_equals(user.password.hashed_password_string().to_string());

                let mut email = Unchanged(found.email.to_string());
                email.set_if_not_equals(user.email.to_string());

                let mut state = Unchanged(found.state.into());
                state.set_if_not_equals(user.state.into());

                let model = ActiveModel {
                    id: Unchanged(found.id),
                    username,
                    password,
                    email,
                    state,
                    create_at: Unchanged(found.created_at),
                    update_at: Set(Some(OffsetDateTime::now_utc())),
                };

                if found.security_events != user.security_events
                    && let Some(event) = user.security_events.pop()
                {
                    let event_model = user_security_event_do::ActiveModel {
                        id: Set(event.id),
                        user_id: Set(event.user_id),
                        event_type: Set(event.event_type.into()),
                        message: Set(event.message),
                        create_at: Set(event.created_at),
                        update_at: NotSet,
                    };
                    event_model.insert(self.conn).await?;
                }

                model.update(self.conn).await.map(model_to_entity)?
            }
            None => Err(ApiError::new(Cause::ClientBadRequest, "User not found")),
        }
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

/// 为 事务&非事务链接 共享
/// 事务内则依赖其直接返回的快照来对比差距
async fn find_by_username(
    conn: &impl sea_orm::ConnectionTrait,
    username: &str,
) -> Result<Option<User>, ApiError> {
    let user = Entity::find()
        .filter(Column::Username.eq(username))
        .one(conn)
        .await?
        .map(model_to_entity)
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
    let email = Email::new(m.email.as_str())?;
    Ok(User {
        id: m.id,
        username,
        password,
        email,
        state: m.state.into(),
        created_at: m.create_at,
        security_events: vec![],
    })
}
