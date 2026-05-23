use sea_orm::entity::prelude::*;
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "post_attachment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub post_id: Uuid,
    #[sea_orm(primary_key)]
    pub attachment_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post_po::Entity",
        from = "Column::PostId",
        to = "super::post_po::Column::Id"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::attachment_po::Entity",
        from = "Column::AttachmentId",
        to = "super::attachment_po::Column::Id"
    )]
    Attachment,
}

impl Related<super::post_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::attachment_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attachment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
