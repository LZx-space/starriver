use sea_orm::DerivePrimaryKey;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::time::OffsetDateTime;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use starriver_identity_domain::user::value_object::UserState;
use uuid::Uuid;

/// 用户
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    #[sea_orm(unique)]
    pub email: String,
    pub state: UserStateDo,
    pub bad_password_window_start: Option<OffsetDateTime>,
    // 注意sea-orm不支持u8,这里用i16接受以不污染外部配置文件及实体，所以他们
    pub bad_password_attempts: i16,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum UserStateDo {
    #[default]
    #[sea_orm(num_value = 0)]
    Active, // 正常
    #[sea_orm(num_value = 1)]
    Locked, // 临时锁定
    #[sea_orm(num_value = 2)]
    Disabled, // 禁用/暂停
}

/////////////////////////////////////////////////////////////
impl From<UserStateDo> for UserState {
    fn from(value: UserStateDo) -> Self {
        match value {
            UserStateDo::Active => UserState::Active,
            UserStateDo::Locked => UserState::Locked,
            UserStateDo::Disabled => UserState::Disabled,
        }
    }
}

impl From<UserState> for UserStateDo {
    fn from(value: UserState) -> Self {
        match value {
            UserState::Active => UserStateDo::Active,
            UserState::Locked => UserStateDo::Locked,
            UserState::Disabled => UserStateDo::Disabled,
        }
    }
}
