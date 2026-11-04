use sea_orm::entity::prelude::*;
use starriver_identity_application::dto::user_dto::input::SecurityEventType;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "security_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub event_type: SecurityEventTypePo,
    pub message: String,
    pub occurred_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum SecurityEventTypePo {
    #[sea_orm(num_value = 0)]
    LoginSucceeded,
    #[sea_orm(num_value = 1)]
    LoginWithBadPassword,
    #[sea_orm(num_value = 2)]
    Userlocked,
    #[sea_orm(num_value = 3)]
    PasswordChanged,
    #[sea_orm(num_value = 4)]
    PasswordReset,
}

impl From<SecurityEventTypePo> for SecurityEventType {
    fn from(event_type: SecurityEventTypePo) -> Self {
        match event_type {
            SecurityEventTypePo::LoginSucceeded => Self::LoginSucceeded,
            SecurityEventTypePo::LoginWithBadPassword => Self::LoginWithBadPassword,
            SecurityEventTypePo::Userlocked => Self::UserLocked,
            SecurityEventTypePo::PasswordChanged => Self::PasswordChanged,
            SecurityEventTypePo::PasswordReset => Self::PasswordReset,
        }
    }
}

impl From<SecurityEventType> for SecurityEventTypePo {
    fn from(event_type: SecurityEventType) -> Self {
        match event_type {
            SecurityEventType::LoginSucceeded => Self::LoginSucceeded,
            SecurityEventType::LoginWithBadPassword => Self::LoginWithBadPassword,
            SecurityEventType::UserLocked => Self::Userlocked,
            SecurityEventType::PasswordChanged => Self::PasswordChanged,
            SecurityEventType::PasswordReset => SecurityEventTypePo::PasswordReset,
        }
    }
}
