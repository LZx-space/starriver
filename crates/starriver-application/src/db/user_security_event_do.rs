use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use starriver_domain::user::value_object::SecurityEventType;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "user_security_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: SecurityEventTypeDo,
    pub message: String,
    pub create_at: OffsetDateTime,
    pub update_at: Option<OffsetDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_do::Entity",
        from = "Column::UserId",
        to = "super::user_do::Column::Id"
    )]
    User,
}

impl Related<super::user_do::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

/////////////////////////////////////////////////////////////////
#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum SecurityEventTypeDo {
    #[default]
    #[sea_orm(num_value = 0)]
    TryLoginWithBadPwd,
    #[sea_orm(num_value = 1)]
    PasswordChanged,
}

impl From<SecurityEventTypeDo> for SecurityEventType {
    fn from(event_type: SecurityEventTypeDo) -> Self {
        match event_type {
            SecurityEventTypeDo::TryLoginWithBadPwd => Self::TryLoginWithBadPwd,
            SecurityEventTypeDo::PasswordChanged => Self::PasswordChanged,
        }
    }
}

impl From<SecurityEventType> for SecurityEventTypeDo {
    fn from(event_type: SecurityEventType) -> Self {
        match event_type {
            SecurityEventType::TryLoginWithBadPwd => Self::TryLoginWithBadPwd,
            SecurityEventType::PasswordChanged => Self::PasswordChanged,
        }
    }
}
