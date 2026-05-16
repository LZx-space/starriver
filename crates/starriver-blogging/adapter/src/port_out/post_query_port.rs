use sea_orm::{
    ActiveEnum, ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait,
};
use starriver_blogging_application::{
    dto::post_dto::{
        req::PageQuery,
        res::{PostDetail, PostExcerpt},
    },
    error::CtxError,
    port_out::post_query_port::PostQueryPort,
};
use starriver_shared_base::{
    dto::PageResult,
    error::QueryError,
    html_utils::{DefaultExcerptor, Excerptor},
};
use uuid::Uuid;

use crate::port_out::po::{
    category_po,
    post_po::{Column, Entity, PostStatePo, Relation},
};

pub struct DefaultPostQueryPort {
    conn: DatabaseConnection,
}

impl PostQueryPort for DefaultPostQueryPort {
    async fn paginate(&self, q: PageQuery) -> Result<PageResult<PostExcerpt>, QueryError> {
        let mut cond = Condition::all();
        if q.published_only {
            cond = cond.add(Column::State.eq(PostStatePo::Published));
        }
        if let Some(category_id) = q.category_id {
            cond = cond.add(Column::CategoryId.eq(category_id));
        }
        let articles = Entity::find()
            .select_only()
            .columns([
                Column::Id,
                Column::Title,
                Column::Content,
                Column::State,
                Column::PublishedAt,
                Column::CreatedAt,
                Column::UpdatedAt,
            ])
            .join(JoinType::LeftJoin, Relation::Category.def())
            .column_as(category_po::Column::Name, "category")
            .filter(cond.clone())
            .offset(q.page * q.page_size)
            .limit(q.page_size)
            .all(&self.conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?
            .into_iter()
            .map(|mut e| PostExcerpt {
                id: e.id,
                title: e.title,
                excerpt: DefaultExcerptor::excerpt(&e.content, 200),
                state: e.state.to_value(),
                category: todo!(),
                published_at: e.published_at,
                created_at: e.created_at,
                updated_at: e.updated_at,
            })
            .collect::<Vec<_>>();
        let record_total = Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(cond)
            .count(&self.conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;
        Ok(PageResult::new(q.page, q.page_size, record_total, articles))
    }

    async fn find_detail(&self, id: Uuid) -> Result<Option<PostDetail>, QueryError> {
        let post = Entity::find_by_id(id)
            .select_only()
            .columns([
                Column::Id,
                Column::Title,
                Column::Content,
                Column::State,
                Column::PublishedAt,
                Column::CreatedAt,
                Column::UpdatedAt,
            ])
            .column_as(Column::Id, "article_id")
            .join(JoinType::LeftJoin, Relation::Category.def())
            .columns([category_po::Column::Id, category_po::Column::Name])
            .one(&self.conn)
            .await
            .map(|mut e| PostDetail {
                id: e.id,
                title: e.title,
                excerpt: DefaultExcerptor::excerpt(&e.content, 200),
                state: e.state.to_value(),
                category: todo!(),
                published_at: e.published_at,
                created_at: e.created_at,
                updated_at: e.updated_at,
                content: todo!(),
                attachments: todo!(),
            })
            .map_err(|e| QueryError::DbError(e.to_string()))?;
        Ok(Some(post))
    }
}
