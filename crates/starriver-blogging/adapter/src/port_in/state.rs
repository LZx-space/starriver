use std::{sync::Arc, time::Duration};

use axum::extract::FromRef;
use moka::future::Cache;
use sea_orm::DatabaseConnection;
use starriver_blogging_application::{
    dto::{
        category_dto::res::CategoryDetailDto,
        post_dto::res::{PostDetailDto, PostExcerptDto},
    },
    use_case::{
        attachement_interactor::AttachmentApplication, category_interactor::CategoryApplication,
        post_interactor::PostApplication,
    },
};
use starriver_blogging_domain::attachment::factory::AttachmentFactory;
use starriver_shared_base::dto::PageResult;
use starriver_shared_framework::{
    config::{Auth, Uploads},
    upload_file::DefaultUploadLocationResolver,
};
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

/// 应用的各个状态
#[derive(Clone)]
pub struct BloggingState {
    pub auth: Arc<Auth>,
    pub uploads: Arc<Uploads>,
    pub upload_file_url_builder: Arc<DefaultUploadLocationResolver>,
    pub post_service:
        Arc<PostApplication<DatabaseConnection, DefaultPostQuery, DefaultPostRepository>>,
    pub category_service: Arc<
        CategoryApplication<DatabaseConnection, DefaultCategoryQuery, DefaultCategoryRepository>,
    >,
    pub attachment_service: Arc<
        AttachmentApplication<
            DatabaseConnection,
            DefaultAttachmentRepository,
            DefaultFileTypeChecker,
            DefaultUploadLocationResolver,
        >,
    >,
}

impl BloggingState {
    pub async fn new(
        conn: DatabaseConnection,
        auth: Arc<Auth>,
        uploads: Arc<Uploads>,
    ) -> Result<Self, String> {
        let upload_file_url_builder = Arc::new(DefaultUploadLocationResolver::new(uploads.clone()));
        let caches = post_caches();
        let post_service = PostApplication::new(
            conn.clone(),
            DefaultPostQuery::new(upload_file_url_builder.clone(), caches.clone()),
            DefaultPostRepository::new(caches.clone()),
        )
        .into();

        let category_list_cache = category_list_cache();
        let category_service = CategoryApplication::new(
            conn.clone(),
            DefaultCategoryQuery::new(category_list_cache.clone()),
            DefaultCategoryRepository::new(category_list_cache.clone()),
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

////////////////////////////////////////////////////////////////////

pub type CatagoryListCache = Cache<(), Vec<CategoryDetailDto>>;

pub const CACHE_KEY_CATEGORY_LIST: () = ();

fn category_list_cache() -> CatagoryListCache {
    Cache::builder()
        .time_to_live(Duration::from_hours(24))
        .build()
}

// -------------------

#[derive(Clone)]
pub struct PostCaches {
    pub page: PostPageCache,
    pub detail: PostDetailCache,
}

impl PostCaches {
    /// 增删改后统一清除所有帖子相关缓存
    pub fn invalidate_all(&self) {
        self.page.invalidate_all();
        self.detail.invalidate_all();
    }
}

pub fn post_caches() -> PostCaches {
    PostCaches {
        page: Cache::builder()
            .time_to_live(Duration::from_hours(1))
            .build(),
        detail: Cache::builder()
            .time_to_live(Duration::from_hours(24))
            .build(),
    }
}

pub type PostPageCache = Cache<PostPageKey, PageResult<PostExcerptDto>>;
pub type PostDetailCache = Cache<Uuid, Option<PostDetailDto>>;

/// 帖子分页查询缓存键，枚举具体参数而不用请求结构体避免添加额外参数时不适合做缓存而注意不到
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PostPageKey {
    pub page: u64,
    pub page_size: u64,
    pub published_only: bool,
    pub category_id: Option<Uuid>,
}
