use sea_orm::DerivePrimaryKey;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::time::OffsetDateTime;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use starriver_identity_domain::user::value_object::AccountStatus;
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
    pub status: AccountStatusDb,
    pub password_locked_until: Option<OffsetDateTime>,
    pub password_window_start: Option<OffsetDateTime>,
    // 注意 sqlx-postgres 不支持读取 u8（仅写入可用），这里用 i16 承接以不污染外部配置文件及实体
    pub password_attempts: i16,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum AccountStatusDb {
    #[default]
    #[sea_orm(num_value = 0)]
    Active, // 正常
    #[sea_orm(num_value = 1)]
    Disabled, // 禁用/暂停
    #[sea_orm(num_value = 2)]
    Deleted, // 已删除
}

/////////////////////////////////////////////////////////////
impl From<AccountStatusDb> for AccountStatus {
    fn from(value: AccountStatusDb) -> Self {
        match value {
            AccountStatusDb::Active => AccountStatus::Active,
            AccountStatusDb::Disabled => AccountStatus::Disabled,
            AccountStatusDb::Deleted => AccountStatus::Deleted,
        }
    }
}

impl From<AccountStatus> for AccountStatusDb {
    fn from(value: AccountStatus) -> Self {
        match value {
            AccountStatus::Active => AccountStatusDb::Active,
            AccountStatus::Disabled => AccountStatusDb::Disabled,
            AccountStatus::Deleted => AccountStatusDb::Deleted,
        }
    }
}
