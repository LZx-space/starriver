use sea_orm::{ActiveValue::Set, entity::prelude::*};
use starriver_domain::article::entity::Attachment;
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "article_attachment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub article_id: Uuid,
    pub create_at: OffsetDateTime,
    pub update_at: Option<OffsetDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl From<Attachment> for ActiveModel {
    fn from(att: Attachment) -> Self {
        let att = att.dissolve();
        Self {
            id: Set(att.0),
            article_id: Set(att.1),
            create_at: Set(att.2),
            update_at: Set(att.3),
        }
    }
}
