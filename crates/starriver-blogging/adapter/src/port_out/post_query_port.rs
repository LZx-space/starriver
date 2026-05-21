use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, PaginatorTrait, QueryFilter,
    QuerySelect, RelationTrait,
};
use starriver_blogging_application::{
    dto::post_dto::{
        req::PageQuery,
        res::{PostDetailDto, PostExcerptDto},
    },
    port_out::post_query_port::PostQueryPort,
};
use starriver_shared_base::{
    dto::{IdName, IdValue, PageResult},
    error::QueryError,
    html_utils::{DefaultExcerptor, Excerptor},
};
use uuid::Uuid;

use crate::port_out::{
    dto::post_dto::{PostDetailRow, PostExcerptRow},
    po::{
        attachment_po, category_po, post_attachment_po,
        post_po::{Column, Entity, PostStatePo, Relation},
    },
};

pub struct DefaultPostQueryPort {
    conn: DatabaseConnection,
}

impl DefaultPostQueryPort {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl PostQueryPort for DefaultPostQueryPort {
    async fn paginate(&self, q: PageQuery) -> Result<PageResult<PostExcerptDto>, QueryError> {
        let mut cond = Condition::all();
        if q.published_only {
            cond = cond.add(Column::State.eq(PostStatePo::Published));
        }
        if let Some(category_id) = q.category_id {
            cond = cond.add(Column::CategoryId.eq(category_id));
        }
        let posts = Entity::find()
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
            .into_model::<PostExcerptRow>()
            .all(&self.conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?
            .into_iter()
            .map(|mut e| {
                e.excerpt = DefaultExcerptor::excerpt(&e.excerpt, 200);
                e.into()
            })
            .collect::<Vec<_>>();
        let record_total = Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(cond)
            .count(&self.conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;
        Ok(PageResult::new(q.page, q.page_size, record_total, posts))
    }

    async fn find_detail(&self, id: Uuid) -> Result<Option<PostDetailDto>, QueryError> {
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
            .join(JoinType::LeftJoin, Relation::Category.def())
            .column_as(category_po::Column::Id, "category_id")
            .column_as(category_po::Column::Name, "category_name")
            .into_model::<PostDetailRow>()
            .one(&self.conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;

        let Some(post) = post else {
            return Ok(None);
        };

        // 2. 附件（零到多行）
        let attachments: Vec<IdValue<_, _>> = post_attachment_po::Entity::find()
            .filter(post_attachment_po::Column::PostId.eq(id))
            .find_with_related(attachment_po::Entity)
            .all(&self.conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?
            .into_iter()
            .flat_map(|(_, attachments)| attachments)
            .map(|a| IdValue {
                id: a.id,
                value: a.file_name,
            })
            .collect();

        Ok(Some(PostDetailDto {
            id,
            title: post.title,
            content: post.content,
            state: post.state,
            category: IdName {
                id: post.category_id,
                name: post.category_name,
            },
            attachments,
            published_at: post.published_at,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }))
    }
}
