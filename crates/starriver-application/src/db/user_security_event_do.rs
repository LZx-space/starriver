use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use time::OffsetDateTime;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "public", table_name = "user_security_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: i16,
    pub message: String,
    pub create_at: OffsetDateTime,
    pub update_at: Option<OffsetDateTime>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub author: HasOne<super::user_do::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
