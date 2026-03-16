use sea_orm::DerivePrimaryKey;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::time::OffsetDateTime;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
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
    pub create_at: OffsetDateTime,
    pub update_at: Option<OffsetDateTime>,

    #[sea_orm(has_many)]
    pub security_events: HasMany<super::user_security_event_do::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
