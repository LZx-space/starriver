use sea_orm::DerivePrimaryKey;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::time::OffsetDateTime;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use starriver_domain::user::value_object::UserState;
use uuid::Uuid;

/// 用户
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    pub state: UserStateDo,
    pub create_at: OffsetDateTime,
    pub update_at: Option<OffsetDateTime>,
    #[sea_orm(has_many)]
    pub security_events: HasMany<super::user_security_event_do::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

/////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum UserStateDo {
    #[default]
    #[sea_orm(num_value = 0)]
    Inactive, // 未激活/待验证
    #[sea_orm(num_value = 1)]
    Active, // 正常
    #[sea_orm(num_value = 2)]
    Locked, // 临时锁定
    #[sea_orm(num_value = 3)]
    Disabled, // 禁用/暂停
}

/////////////////////////////////////////////////////////////

impl From<UserStateDo> for UserState {
    fn from(value: UserStateDo) -> Self {
        match value {
            UserStateDo::Active => UserState::Active,
            UserStateDo::Inactive => UserState::Inactive,
            UserStateDo::Locked => UserState::Locked,
            UserStateDo::Disabled => UserState::Disabled,
        }
    }
}

impl From<UserState> for UserStateDo {
    fn from(value: UserState) -> Self {
        match value {
            UserState::Active => UserStateDo::Active,
            UserState::Inactive => UserStateDo::Inactive,
            UserState::Locked => UserStateDo::Locked,
            UserState::Disabled => UserStateDo::Disabled,
        }
    }
}
