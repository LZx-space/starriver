use std::sync::Arc;

use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, JoinType, Order, PaginatorTrait,
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
    port::post_query::PostQuery,
};
use starriver_blogging_domain::post::value_object::PostState;
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
    port_in::state::{PostCaches, PostPageKey},
    port_out::persistence::po::{
        attachment_po, category_po, post_attachment_po,
        post_po::{Column, Entity, PostStatePo, Relation},
    },
};

pub struct DefaultPostQuery {
    file_url_builder: Arc<DefaultUploadLocationResolver>,
    caches: PostCaches,
}

impl DefaultPostQuery {
    pub fn new(file_url_builder: Arc<DefaultUploadLocationResolver>, caches: PostCaches) -> Self {
        Self {
            file_url_builder,
            caches,
        }
    }
}

impl PostQuery for DefaultPostQuery {
    async fn paginate<C: ConnectionTrait>(
        &self,
        conn: &C,
        q: PageQuery,
    ) -> Result<PageResult<PostExcerptDto>, QueryError> {
        let key = PostPageKey {
            page: q.page,
            page_size: q.page_size,
            published_only: q.published_only,
            category_id: q.category_id,
        };
        self.caches
            .page
            .try_get_with(key, async {
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
                    .all(conn)
                    .await?
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
                    .count(conn)
                    .await?;
                Ok(PageResult::new(q.page, q.page_size, record_total, posts))
            })
            .await
            .map_err(|e: Arc<DbErr>| QueryError::DbError(e.to_string()))
    }

    async fn find_detail<C: ConnectionTrait>(
        &self,
        conn: &C,
        id: Uuid,
    ) -> Result<Option<PostDetailDto>, QueryError> {
        self.caches
            .detail
            .try_get_with(id, async {
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
                    .one(conn)
                    .await?;

                let Some(post) = post else {
                    return Ok(None);
                };

                let attachments: Vec<AttachmentDto> = post_attachment_po::Entity::find()
                    .filter(post_attachment_po::Column::PostId.eq(id))
                    .find_with_related(attachment_po::Entity)
                    .all(conn)
                    .await?
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
                    state: PostState::from(post.state).to_string(),
                    category: IdName {
                        id: post.category_id,
                        name: post.category_name,
                    },
                    attachments,
                    published_at: post.published_at,
                    created_at: post.created_at,
                    updated_at: post.updated_at,
                }))
            })
            .await
            .map_err(|e: Arc<DbErr>| QueryError::DbError(e.to_string()))
    }
}
