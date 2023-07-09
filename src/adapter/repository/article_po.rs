use sea_orm::entity::prelude::*;

/// 文章
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "article")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub title: String,

    #[sea_orm(column_type = "Text")]
    pub body: String,

    pub author_id: String,

    pub create_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tag_po::Entity")]
    Tag,
}

impl Related<super::tag_po::Entity> for Entity {
    fn to() -> RelationDef {
        super::article_tag_po::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::article_tag_po::Relation::Article.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
