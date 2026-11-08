use sea_orm::entity::prelude::*;
use starriver_identity_application::dto::user_dto::input::SecurityEventKind;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "security_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub kind: SecurityEventKindDb,
    pub payload: Option<serde_json::Value>,
    pub occurred_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum SecurityEventKindDb {
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

impl From<SecurityEventKindDb> for SecurityEventKind {
    fn from(event_type: SecurityEventKindDb) -> Self {
        match event_type {
            SecurityEventKindDb::LoginSucceeded => Self::LoginSucceeded,
            SecurityEventKindDb::LoginWithBadPassword => Self::LoginWithBadPassword,
            SecurityEventKindDb::Userlocked => Self::UserLocked,
            SecurityEventKindDb::PasswordChanged => Self::PasswordChanged,
            SecurityEventKindDb::PasswordReset => Self::PasswordReset,
        }
    }
}

impl From<SecurityEventKind> for SecurityEventKindDb {
    fn from(event_type: SecurityEventKind) -> Self {
        match event_type {
            SecurityEventKind::LoginSucceeded => Self::LoginSucceeded,
            SecurityEventKind::LoginWithBadPassword => Self::LoginWithBadPassword,
            SecurityEventKind::UserLocked => Self::Userlocked,
            SecurityEventKind::PasswordChanged => Self::PasswordChanged,
            SecurityEventKind::PasswordReset => SecurityEventKindDb::PasswordReset,
        }
    }
}
