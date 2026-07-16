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
    dto::{IdName, PageResult, PageSearch},
    error::QueryError,
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
        let paginator = Entity::find()
            .select_only()
            .columns([
                Column::Id,
                Column::Title,
                Column::Excerpt,
                Column::State,
                Column::PublishedAt,
                Column::CreatedAt,
                Column::UpdatedAt,
            ])
            .join(JoinType::LeftJoin, Relation::Category.def())
            .column_as(category_po::Column::Name, "category")
            .filter(cond)
            .order_by_with_nulls(order, Order::Desc, NullOrdering::Last)
            .into_model::<PostExcerptRow>()
            .paginate(conn, q.page_size);
        let posts = paginator
            .fetch_page(q.page)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?
            .into_iter()
            .map(|e| e.into())
            .collect::<Vec<_>>();
        let total_items = paginator
            .num_items()
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;
        Ok(PageResult::new(q.page, q.page_size, total_items, posts))
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
        q: PageSearch,
    ) -> Result<PageResult<PostSearchDto>, QueryError> {
        if q.q.is_empty() {
            return Ok(PageResult::new(q.page, q.page_size, 0, vec![]));
        }

        // pgroonga扩展实现搜索功能
        // 数据量极小时可能会由于不走全文索引而score为0
        // WHERE title OR content会导致不走全文索引，因此UNION ALL
        let rows = PostSearchRow::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH
               	search_kw AS (
              		SELECT pgroonga_query_extract_keywords($3) AS kw
               	),
               	unioned AS (
               	    SELECT
             			id,
             			title,
             			content,
             			published_at,
             			category_id,
             			pgroonga_score(tableoid, ctid) AS score
              		FROM
             			post
              		WHERE
             			state = 1 AND title &@~ $3
               	    UNION ALL
               	    SELECT
             			id,
             			title,
             			content,
             			published_at,
             			category_id,
             			pgroonga_score(tableoid, ctid) AS score
              		FROM
             			post
              		WHERE
             			state = 1 AND content &@~ $3
               	),
               	ranked AS (
               	    SELECT
                        *,
                	    ROW_NUMBER() OVER (PARTITION BY id ORDER BY score DESC) AS rn
               	    FROM
                        unioned
               	),
               	filtered AS (
               	    SELECT
               	        id,
               	        title,
               	        regexp_replace(
               	            regexp_replace(content, '<[^>]*>', '', 'g'),
               	            '&[^;]+;',
               	            '',
               	            'g'
               	        ) AS clean_content,
             			published_at,
                        category_id,
                        score
               	    FROM
                        ranked
               	    WHERE
                        rn = 1
               	)
            SELECT
                filtered.id,
                filtered.title,
                COALESCE(
                    NULLIF(
                        (pgroonga_snippet_html(filtered.clean_content, (SELECT kw FROM search_kw), 100))[1],
                        ''
                    ),
                    left(filtered.clean_content, 100)
                ) AS snippet,
                filtered.published_at,
                filtered.score,
                category.name AS category,
                COUNT(*) OVER() AS total_count
            FROM
               	filtered
               	LEFT JOIN category ON filtered.category_id = category.id
            ORDER BY
               	score DESC,
                published_at DESC NULLS LAST
            OFFSET
                $1
            LIMIT
               	$2;
            "#,
            [q.page.into(), q.page_size.into(), q.q.into()],
        ))
        .all(conn)
        .await
        .map_err(|e| QueryError::DbError(e.to_string()))?;

        if rows.is_empty() {
            return Ok(PageResult::new(q.page, q.page_size, 0, vec![]));
        }

        let total_count = rows[0].total_count;
        let items = rows
            .into_iter()
            .map(|e| PostSearchDto {
                id: e.id,
                title: e.title,
                snippet: e.snippet,
                category: e.category,
                published_at: e.published_at,
                score: e.score,
            })
            .collect();
        Ok(PageResult::new(
            q.page,
            q.page_size,
            total_count as u64,
            items,
        ))
    }
}
