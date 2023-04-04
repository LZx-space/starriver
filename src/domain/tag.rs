use sea_orm::prelude::Uuid;

/// 标签
#[derive(Debug)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}
