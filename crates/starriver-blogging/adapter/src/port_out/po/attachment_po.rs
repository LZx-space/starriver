use sea_orm::entity::prelude::*;
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "attachment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub file_name: String,
    pub file_size: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 附件被哪些帖子引用（通过 post_attachment 表）
    #[sea_orm(has_many = "super::post_attachment_po::Entity")]
    PostAttachment,
}

impl Related<super::post_attachment_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostAttachment.def()
    }
}

// 可选的“反向”多对多导航
impl Related<super::post_po::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_attachment_po::Relation::Post.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::post_attachment_po::Relation::Attachment.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
