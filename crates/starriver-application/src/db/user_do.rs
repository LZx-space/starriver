use sea_orm::DerivePrimaryKey;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::time::OffsetDateTime;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
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

    pub create_at: OffsetDateTime,

    pub update_at: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_security_event_do::Entity")]
    UserSecurityEvent,
}

impl Related<super::user_security_event_do::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSecurityEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
