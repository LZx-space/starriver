use sea_orm::entity::prelude::*;

/// 标签
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,

    pub create_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::adapter::repository::blog::po::article::Entity")]
    Article,
}

impl Related<crate::adapter::repository::blog::po::article::Entity> for Entity {
    fn to() -> RelationDef {
        super::article_tag::Relation::Article.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::article_tag::Relation::Tag.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
