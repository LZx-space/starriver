use sea_orm::entity::prelude::*;
use starriver_identity_domain::security_event::value_object::SecurityEventType;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "security_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: SecurityEventTypePo,
    pub message: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_po::Entity",
        from = "Column::UserId",
        to = "super::user_po::Column::Id"
    )]
    User,
}

/////////////////////////////////////////////////////////////////
#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum SecurityEventTypePo {
    #[default]
    #[sea_orm(num_value = 0)]
    TryLoginWithBadPwd,
    #[sea_orm(num_value = 1)]
    UserUnlocked,
    #[sea_orm(num_value = 2)]
    PasswordChanged,
}

impl From<SecurityEventTypePo> for SecurityEventType {
    fn from(event_type: SecurityEventTypePo) -> Self {
        match event_type {
            SecurityEventTypePo::TryLoginWithBadPwd => Self::TryLoginWithBadPwd,
            SecurityEventTypePo::UserUnlocked => Self::UserUnlocked,
            SecurityEventTypePo::PasswordChanged => Self::PasswordChanged,
        }
    }
}

impl From<SecurityEventType> for SecurityEventTypePo {
    fn from(event_type: SecurityEventType) -> Self {
        match event_type {
            SecurityEventType::TryLoginWithBadPwd => Self::TryLoginWithBadPwd,
            SecurityEventType::UserUnlocked => Self::UserUnlocked,
            SecurityEventType::PasswordChanged => Self::PasswordChanged,
        }
    }
}
