use sea_orm::entity::prelude::*;
use sea_orm::{DeriveActiveEnum, EnumIter};
use starriver_blogging_domain::post::value_object::PostState;
use time::OffsetDateTime;

/// 博客
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    #[sea_orm(column_type = "Text")]
    pub excerpt: String,
    pub state: PostStatePo,
    pub author_id: Uuid,
    pub category_id: Uuid,
    #[sea_orm(indexed)]
    pub published_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category_po::Entity",
        from = "Column::CategoryId",
        to = "super::category_po::Column::Id"
    )]
    Category,

    // 到关联表 post_attachment 的一对多关系
    #[sea_orm(has_many = "super::post_attachment_po::Entity")]
    PostAttachment,
}

// 实现到 Category 的默认导航
impl Related<super::category_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

// 实现到 PostAttachment 的默认导航
impl Related<super::post_attachment_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostAttachment.def()
    }
}

// 多对多导航：Post -> Attachment（通过 post_attachment 表）
impl Related<super::attachment_po::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_attachment_po::Relation::Attachment.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::post_attachment_po::Relation::Post.def().rev())
    }
}

//////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum PostStatePo {
    #[sea_orm(num_value = 0)]
    #[default]
    Draft,
    #[sea_orm(num_value = 1)]
    Published,
    #[sea_orm(num_value = 2)]
    Archived,
}

//////////////////////////////////////////////////////////

impl From<PostStatePo> for PostState {
    fn from(value: PostStatePo) -> Self {
        match value {
            PostStatePo::Draft => PostState::Draft,
            PostStatePo::Published => PostState::Published,
            PostStatePo::Archived => PostState::Archived,
        }
    }
}

impl From<PostState> for PostStatePo {
    fn from(value: PostState) -> Self {
        match value {
            PostState::Draft => PostStatePo::Draft,
            PostState::Published => PostStatePo::Published,
            PostState::Archived => PostStatePo::Archived,
        }
    }
}
