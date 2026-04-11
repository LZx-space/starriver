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
use sea_orm::sea_query::Cond;
use starriver_domain::user::entity::SecurityEvent;
use starriver_domain::user::entity::User;
use starriver_domain::user::factory::UserFactory;
use starriver_domain::user::repository::UserRepository;
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::error::Cause;
use starriver_infrastructure::util::db::TransactionalConn;
use time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct DefaultUserRepository<T> {
    conn: T,
    factory: UserFactory,
}

impl<T> DefaultUserRepository<T>
where
    T: TransactionalConn,
{
    /// 普通链接获取开启事物的链接
    pub fn new(conn: T, factory: UserFactory) -> DefaultUserRepository<T> {
        Self { conn, factory }
    }

    pub fn conn(self) -> T {
        self.conn
    }
}

impl<T> UserRepository for DefaultUserRepository<T>
where
    T: TransactionalConn,
{
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        find_by_username(&self.conn, username, &self.factory).await
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, ApiError> {
        let user = Entity::find_by_id(user_id)
            .one(&self.conn)
            .await?
            .map(|e| model_to_entity(e, &self.factory))
            .transpose()?;
        Ok(user)
    }

    async fn insert(&self, user: User) -> Result<User, ApiError> {
        let (id, username, password, email, _, _, _) = user.dissolve();
        let username = username.as_str();
        let found = self.find_by_username(username).await?;
        if found.is_some() {
            return Err(ApiError::new(
                Cause::ClientBadRequest,
                format!("username[{}] already exists", username),
            ));
        }
        ActiveModel {
            id: Set(id),
            username: Set(username.to_string()),
            password: Set(password.as_str().to_string()),
            email: Set(email.to_string()),
            state: Set(UserStateDo::Active),
            create_at: Set(OffsetDateTime::now_utc()),
            update_at: NotSet,
        }
        .insert(&self.conn)
        .await
        .map(|e| model_to_entity(e, &self.factory))?
    }

    async fn delete(&self, user_id: uuid::Uuid) -> Result<bool, ApiError> {
        let result = Entity::delete_by_id(user_id)
            .exec(&self.conn)
            .await?
            .rows_affected
            > 0;
        Ok(result)
    }

    async fn update(&self, user: User) -> Result<User, ApiError> {
        let (id, new_username, new_password, new_email, new_state, _, mut new_security_events) =
            user.dissolve();
        let username = new_username.as_str();
        match find_by_username(&self.conn, username, &self.factory).await? {
            Some(found) => {
                let (_, username, password, email, state, create_at, security_events) =
                    found.dissolve();
                // 更新事件，一次请求仅可能新增一条记录
                if security_events != new_security_events
                    && let Some(event) = new_security_events.pop()
                {
                    let (id, user_id, event_type, message, created_at) = event.dissolve();
                    let event_model = user_security_event_do::ActiveModel {
                        id: Set(id),
                        user_id: Set(user_id),
                        event_type: Set(event_type.into()),
                        message: Set(message),
                        create_at: Set(created_at),
                        update_at: NotSet,
                    };
                    event_model.insert(&self.conn).await?;
                }

                // 更新用户
                let mut username = Unchanged(username.as_str().to_string());
                username.set_if_not_equals(new_username.as_str().to_string());

                let mut password = Unchanged(password.as_str().to_string());
                password.set_if_not_equals(new_password.as_str().to_string());

                let mut email = Unchanged(email.to_string());
                email.set_if_not_equals(new_email.to_string());

                let mut state = Unchanged(state.into());
                state.set_if_not_equals(new_state.into());

                let model = ActiveModel {
                    id: Unchanged(id),
                    username,
                    password,
                    email,
                    state,
                    create_at: Unchanged(create_at),
                    update_at: Set(Some(OffsetDateTime::now_utc())),
                };

                model
                    .update(&self.conn)
                    .await
                    .map(|e| model_to_entity(e, &self.factory))?
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
    factory: &UserFactory,
) -> Result<Option<User>, ApiError> {
    let user = Entity::find()
        .filter(Column::Username.eq(username))
        .one(conn)
        .await?
        .map(|e| model_to_entity(e, factory))
        .transpose()?;
    if let Some(mut user) = user {
        let mut events: Vec<SecurityEvent> = user_security_event_do::Entity::find()
            .filter({
                // 最近30分钟内的事件，注意当前此处没有与PasswordSpecification同步
                Cond::all()
                    .add(user_security_event_do::Column::UserId.eq(user.id().to_owned()))
                    .add(
                        user_security_event_do::Column::CreateAt
                            .gt(OffsetDateTime::now_utc().saturating_sub(Duration::minutes(30))),
                    )
            })
            .order_by_desc(user_security_event_do::Column::CreateAt) // 按时间倒序取最新
            .all(conn)
            .await?
            .iter()
            .map(|e| {
                SecurityEvent::from_repo(
                    e.id,
                    e.user_id,
                    e.event_type.to_owned().into(),
                    e.message.to_owned(),
                    e.create_at,
                )
            })
            .collect();
        user.security_events().append(&mut events);
        return Ok(Some(user));
    }
    Ok(user)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[inline]
fn model_to_entity(m: Model, factory: &UserFactory) -> Result<User, ApiError> {
    factory.from_repo(
        m.id,
        m.username.as_str(),
        m.password.as_str(),
        m.email.as_str(),
        m.state.into(),
        m.create_at,
        vec![],
    )
}
