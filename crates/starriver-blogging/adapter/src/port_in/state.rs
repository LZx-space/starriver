use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use starriver_blogging_application::{
    dto::{
        category_dto::res::CategoryDetailDto,
        post_dto::res::{PostDetailDto, PostExcerptDto},
    },
    port::post_cache::{PostCaches, PostPageKey},
    use_case::{
        attachement_interactor::AttachmentApplication, category_interactor::CategoryApplication,
        post_interactor::PostApplication,
    },
};
use starriver_blogging_domain::attachment::factory::AttachmentFactory;
use starriver_shared_base::dto::PageResult;
use starriver_shared_framework::{
    cache::DefaultCache,
    config::{Auth, Uploads},
    repository::DefaultConnection,
    upload_file::DefaultUploadLocationResolver,
};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

use crate::port_out::{
    persistence::{
        query::{category_query::DefaultCategoryQuery, post_query::DefaultPostQuery},
        repository::{
            attachment_repository::DefaultAttachmentRepository,
            category_repository::DefaultCategoryRepository, post_repository::DefaultPostRepository,
        },
    },
    service::file_type_checker::DefaultFileTypeChecker,
};

type PostService = PostApplication<
    DefaultConnection,
    DefaultPostQuery,
    DefaultPostRepository,
    DefaultCache<PostPageKey, PageResult<PostExcerptDto>>,
    DefaultCache<Uuid, Option<PostDetailDto>>,
>;

type CategoryService = CategoryApplication<
    DefaultConnection,
    DefaultCategoryQuery,
    DefaultCategoryRepository,
    DefaultCache<(), Vec<CategoryDetailDto>>,
>;

type AttachmentService = AttachmentApplication<
    DefaultConnection,
    DefaultAttachmentRepository,
    DefaultFileTypeChecker,
    DefaultUploadLocationResolver,
>;

/// 应用的各个状态
#[derive(Clone)]
pub struct BloggingState {
    pub auth: Arc<Auth>,
    pub uploads: Arc<Uploads>,
    pub upload_file_url_builder: DefaultUploadLocationResolver,
    pub post_service: Arc<PostService>,
    pub category_service: Arc<CategoryService>,
    pub attachment_service: Arc<AttachmentService>,
}

impl BloggingState {
    pub async fn new(
        conn: DatabaseConnection,
        auth: Arc<Auth>,
        uploads: Arc<Uploads>,
    ) -> Result<Self, String> {
        let upload_file_url_builder = DefaultUploadLocationResolver::new(uploads.clone());
        let conn = DefaultConnection::new(conn);

        let post_cache = PostCaches::new(
            DefaultCache::new(1, Duration::from_hours(1)),
            DefaultCache::new(100, Duration::from_hours(1)),
        );
        let post_service = PostApplication::new(
            conn.clone(),
            DefaultPostQuery::new(upload_file_url_builder.clone()),
            DefaultPostRepository,
            post_cache,
        )
        .into();

        let category_cache = DefaultCache::new(100, Duration::from_hours(1));
        let category_service = CategoryApplication::new(
            conn.clone(),
            DefaultCategoryQuery,
            DefaultCategoryRepository,
            category_cache,
        )
        .into();

        let attachment_service = AttachmentApplication::new(
            conn.clone(),
            DefaultAttachmentRepository,
            AttachmentFactory::new(DefaultFileTypeChecker {}),
            upload_file_url_builder.clone(),
        )
        .into();

        Ok(BloggingState {
            auth,
            uploads,
            upload_file_url_builder,
            post_service,
            category_service,
            attachment_service,
        })
    }
}

////////////////////////////////////////////////////////////////////

impl FromRef<BloggingState> for Arc<Auth> {
    fn from_ref(input: &BloggingState) -> Self {
        input.auth.clone()
    }
}
