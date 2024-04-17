use sea_orm::{ActiveModelBehavior, DatabaseConnection};
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeLocal;
use uuid::Uuid;

use crate::infrastructure::model::page::{PageQuery, PageResult};
use crate::infrastructure::service::dictionary::dictionary_service::DictionaryEntry;

pub struct Repository {
    pub conn: &'static DatabaseConnection,
}

impl Repository {
    pub async fn paging(&self, query: PageQuery) -> Result<PageResult<DictionaryEntry>, DbErr> {
        todo!()
    }

    pub async fn insert(&self, e: DictionaryEntry) -> Option<DbErr> {
        let model = ActiveModel {
            id: Set(e.id),
            value: Set(e.value),
            data_type: Set(e.data_type.to_string()),
            comment: Set(e.comment),
            create_at: Set(e.create_at),
            update_at: Set(None),
        };
        model.insert(self.conn).await.err()
    }

    pub async fn update(&self, e: DictionaryEntry) -> Option<DbErr> {
        todo!()
    }

    pub async fn delete(&self, e: DictionaryEntry) -> Option<DbErr> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "dictionary")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,

    value: String,

    data_type: String,

    comment: String,

    create_at: DateTimeLocal,

    update_at: Option<DateTimeLocal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
