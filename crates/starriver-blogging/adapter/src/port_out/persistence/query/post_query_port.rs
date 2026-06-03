use std::sync::Arc;

use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait, sea_query::NullOrdering,
};
use starriver_blogging_application::{
    dto::{
        attachment_dto::res::AttachmentDto,
        post_dto::{
            req::PageQuery,
            res::{PostDetailDto, PostExcerptDto},
        },
    },
    port_out::post_query_port::PostQueryPort,
};
use starriver_shared_base::{
    dto::{IdName, PageResult},
    error::QueryError,
    html_utils::{DefaultExcerptor, Excerptor},
    upload_file::UploadLocationResolver,
};
use starriver_shared_framework::upload_file::DefaultUploadLocationResolver;
use uuid::Uuid;

use crate::{
    dto::post_dto::{PostDetailRow, PostExcerptRow},
    port_out::persistence::po::{
        attachment_po, category_po, post_attachment_po,
        post_po::{Column, Entity, PostStatePo, Relation},
    },
};

pub struct DefaultPostQueryPort {
    conn: DatabaseConnection,
    file_url_builder: Arc<DefaultUploadLocationResolver>,
}

impl DefaultPostQueryPort {
    pub fn new(
        conn: DatabaseConnection,
        file_url_builder: Arc<DefaultUploadLocationResolver>,
    ) -> Self {
        Self {
            conn,
            file_url_builder,
        }
    }
}

impl PostQueryPort for DefaultPostQueryPort {
    async fn paginate(&self, q: PageQuery) -> Result<PageResult<PostExcerptDto>, QueryError> {
        let mut cond = Condition::all();
        let order;
        if q.published_only {
            cond = cond.add(Column::State.eq(PostStatePo::Published));
            order = Column::PublishedAt;
        } else {
            order = Column::UpdatedAt;
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
            .order_by_with_nulls(order, Order::Desc, NullOrdering::Last)
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
        let attachments: Vec<AttachmentDto> = post_attachment_po::Entity::find()
            .filter(post_attachment_po::Column::PostId.eq(id))
            .find_with_related(attachment_po::Entity)
            .all(&self.conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?
            .into_iter()
            .flat_map(|(_, attachments)| attachments)
            .map(|a| AttachmentDto {
                id: a.id,
                url: self.file_url_builder.url(a.file_name.as_str()),
                file_name: a.file_name,
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
