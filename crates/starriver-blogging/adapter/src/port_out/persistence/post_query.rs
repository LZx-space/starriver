use sea_orm::{
    ColumnTrait, Condition, DbBackend, EntityTrait, FromQueryResult, JoinType, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Statement,
    sea_query::NullOrdering,
};
use starriver_blogging_application::{
    dto::{
        attachment_dto::res::AttachmentDto,
        post_dto::{
            req::PageQuery,
            res::{PostDetailDto, PostExcerptDto, PostSearchDto},
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
use starriver_shared_framework::{
    db::DefaultConnection, upload_file::DefaultUploadLocationResolver,
};
use uuid::Uuid;

use crate::port_out::persistence::{
    dto::post_row::{PostDetailRow, PostExcerptRow, PostSearchRow},
    po::{
        attachment_po, category_po, post_attachment_po,
        post_po::{Column, Entity, PostStatePo, Relation},
    },
};

pub struct DefaultPostQuery {
    file_url_builder: DefaultUploadLocationResolver,
}

impl DefaultPostQuery {
    pub fn new(file_url_builder: DefaultUploadLocationResolver) -> Self {
        Self { file_url_builder }
    }
}

impl PostQuery<DefaultConnection> for DefaultPostQuery {
    async fn paginate(
        &self,
        conn: &DefaultConnection,
        q: PageQuery,
    ) -> Result<PageResult<PostExcerptDto>, QueryError> {
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
            .count(conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;
        Ok(PageResult::new(q.page, q.page_size, record_total, posts))
    }

    async fn find_detail(
        &self,
        conn: &DefaultConnection,
        id: Uuid,
    ) -> Result<Option<PostDetailDto>, QueryError> {
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
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;

        let Some(post) = post else {
            return Ok(None);
        };

        let attachments: Vec<AttachmentDto> = post_attachment_po::Entity::find()
            .filter(post_attachment_po::Column::PostId.eq(id))
            .find_with_related(attachment_po::Entity)
            .all(conn)
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
    }

    async fn search(
        &self,
        conn: &DefaultConnection,
        q: &str,
    ) -> Result<Vec<PostSearchDto>, QueryError> {
        if q.is_empty() {
            return Ok(vec![]);
        }
        let q = q.split_whitespace().collect::<Vec<_>>().join(" OR ");
        // pgroonga扩展实现搜索功能
        let rows = PostSearchRow::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH
                search_kw AS (
                    SELECT pgroonga_query_extract_keywords($1) AS kw
                ),
                filtered_posts AS (
                    SELECT
                        post.id,
                        post.title,
                        post.content,
                        post.published_at,
                        post.category_id,
                        regexp_replace(post.content, '<[^>]*>', '', 'g') AS clean_content
                    FROM post
                    WHERE
                        post.state = 1
                        AND (post.title &@~ $1 OR post.content &@~ $1)
            )
            SELECT
                fp.id,
                fp.title,
                COALESCE(
                    NULLIF(
                        (pgroonga_snippet_html(fp.clean_content, (SELECT kw FROM search_kw), 100))[1],
                        ''
                    ),
                    left(fp.clean_content, 100)
                ) AS snippet,
                fp.published_at,
                category.name AS category
            FROM filtered_posts fp
            LEFT JOIN category ON fp.category_id = category.id
            ORDER BY fp.published_at DESC NULLS LAST
            LIMIT 10;
            "#,
            [q.into()],
        ))
        .all(conn)
        .await
        .map_err(|e| QueryError::DbError(e.to_string()))?;

        let result = rows
            .into_iter()
            .map(|e| PostSearchDto {
                id: e.id,
                title: e.title,
                published_at: e.published_at,
                category: e.category,
                snippet: e.snippet,
                category: e.category,
                published_at: e.published_at,
            })
            .collect();
        Ok(result)
    }
}
