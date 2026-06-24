use sea_orm::DerivePrimaryKey;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::time::OffsetDateTime;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use starriver_identity_domain::user::value_object::LifeCycle;
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
    pub life_cycle: UserLifeCycle,
    pub password_locked_until: Option<OffsetDateTime>,
    pub password_window_start: Option<OffsetDateTime>,
    // 注意sea-orm不支持u8,这里用i16接受以不污染外部配置文件及实体
    pub password_attempts: i16,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum UserLifeCycle {
    #[default]
    #[sea_orm(num_value = 0)]
    Active, // 正常
    #[sea_orm(num_value = 1)]
    Disabled, // 禁用/暂停
    #[sea_orm(num_value = 2)]
    Deleted, // 临时锁定
}

/////////////////////////////////////////////////////////////
impl From<UserLifeCycle> for LifeCycle {
    fn from(value: UserLifeCycle) -> Self {
        match value {
            UserLifeCycle::Active => LifeCycle::Active,
            UserLifeCycle::Disabled => LifeCycle::Disabled,
            UserLifeCycle::Deleted => LifeCycle::Deleted,
        }
    }
}

impl From<LifeCycle> for UserLifeCycle {
    fn from(value: LifeCycle) -> Self {
        match value {
            LifeCycle::Active => UserLifeCycle::Active,
            LifeCycle::Disabled => UserLifeCycle::Disabled,
            LifeCycle::Deleted => UserLifeCycle::Deleted,
        }
    }
}
