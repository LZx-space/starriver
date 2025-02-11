use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelBehavior, DatabaseConnection, QueryOrder};
use std::str::FromStr;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::infrastructure::model::page::{PageQuery, PageResult};
use crate::infrastructure::service::dictionary::dictionary_service::DataType;
use crate::infrastructure::service::dictionary::dictionary_service::DictionaryEntry;

pub struct Repository {
    conn: &'static DatabaseConnection,
}

impl Repository {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        Repository { conn }
    }

    pub async fn paging(&self, query: PageQuery) -> Result<PageResult<DictionaryEntry>, DbErr> {
        let paginator = Entity::find()
            .order_by_asc(Column::Id)
            .paginate(self.conn, query.page_size);
        let num_items = paginator.num_items().await?;

        // Fetch paginated posts
        paginator.fetch_page(query.page).await.map(|v| {
            PageResult::new(
                query.page,
                query.page_size,
                num_items,
                v.iter()
                    .map(|m| {
                        let m = m.clone();
                        let data_type = DataType::from_str(&m.data_type).expect("");
                        DictionaryEntry::new(m.value, data_type, m.comment).expect("")
                    })
                    .collect(),
            )
        })
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

    create_at: OffsetDateTime,

    update_at: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
