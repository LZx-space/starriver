use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use starriver_blogging_application::{
    dto::{
        category_dto::output::CategoryDetailDto,
        post_dto::output::{PostDetailDto, PostExcerptDto},
    },
    port::{category_cache::CategoryListKey, post_cache::PostPageKey},
    use_case::{attachment_interactor, category_interactor, post},
};
use starriver_blogging_domain::attachment::factory::AttachmentFactory;
use starriver_shared_base::{dto::OffsetPage, random::duration_with_jitter};
use starriver_shared_infra::{
    cache::DefaultCache,
    config::{Auth, Uploads},
    db::DefaultConnection,
    upload_file::DefaultUploadLocationResolver,
};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

use crate::{
    config::{BloggingConfig, Cache},
    port_out::{
        persistence::{
            attachment_repository::DefaultAttachmentRepository,
            category_query::DefaultCategoryQuery, category_repository::DefaultCategoryRepository,
            post_query::DefaultPostQuery, post_repository::DefaultPostRepository,
        },
        service::file_type_checker::DefaultFileTypeChecker,
    },
};

type PostCreateInteractor = post::create_interactor::CreateInteractor<
    DefaultConnection,
    DefaultPostQuery,
    DefaultPostRepository,
    DefaultCategoryRepository,
    DefaultCache<PostPageKey, OffsetPage<PostExcerptDto>>,
>;

type PostDeleteInteractor = post::delete_interactor::DeleteInteractor<
    DefaultConnection,
    DefaultPostRepository,
    DefaultCache<PostPageKey, OffsetPage<PostExcerptDto>>,
    DefaultCache<Uuid, Option<PostDetailDto>>,
>;

type PostFindInteractor = post::find_interactor::FindInteractor<
    DefaultConnection,
    DefaultPostQuery,
    DefaultCache<Uuid, Option<PostDetailDto>>,
>;

type PostPaginateInteractor = post::paginate_interactor::PaginateInteractor<
    DefaultConnection,
    DefaultPostQuery,
    DefaultCache<PostPageKey, OffsetPage<PostExcerptDto>>,
>;

type PostSearchInteractor =
    post::search_interactor::SearchInteractor<DefaultConnection, DefaultPostQuery>;

type PostUpdateInteractor = post::update_interactor::UpdateInteractor<
    DefaultConnection,
    DefaultPostRepository,
    DefaultCategoryRepository,
    DefaultCache<PostPageKey, OffsetPage<PostExcerptDto>>,
    DefaultCache<Uuid, Option<PostDetailDto>>,
>;

///////////////////////////////////////////////////////////////////////////////

type CategoryInteractor = category_interactor::CategoryInteractor<
    DefaultConnection,
    DefaultCategoryQuery,
    DefaultCategoryRepository,
    DefaultCache<CategoryListKey, Vec<CategoryDetailDto>>,
    DefaultCache<PostPageKey, OffsetPage<PostExcerptDto>>,
    DefaultCache<Uuid, Option<PostDetailDto>>,
>;

type AttachmentInteractor = attachment_interactor::AttachmentInteractor<
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
    /////////////////////////////////////////////////////////////////
    pub post_create_interactor: Arc<PostCreateInteractor>,
    pub post_delete_interactor: Arc<PostDeleteInteractor>,
    pub post_find_interactor: Arc<PostFindInteractor>,
    pub post_paginate_interactor: Arc<PostPaginateInteractor>,
    pub post_search_interactor: Arc<PostSearchInteractor>,
    pub post_update_interactor: Arc<PostUpdateInteractor>,
    /////////////////////////////////////////////////////////////////
    pub category_interactor: Arc<CategoryInteractor>,
    /////////////////////////////////////////////////////////////////
    pub attachment_interactor: Arc<AttachmentInteractor>,
}

impl BloggingState {
    pub async fn new(
        conn: DatabaseConnection,
        auth: Arc<Auth>,
        uploads: Arc<Uploads>,
        cfg: &BloggingConfig,
    ) -> Result<Self, String> {
        let upload_file_url_builder = DefaultUploadLocationResolver::new(uploads.clone());
        let conn = DefaultConnection::new(conn);

        let cache_cfg = &cfg.cache;
        let post_page_cache = DefaultCache::new(
            cache_cfg.post_page_cache_max_capacity,
            cache_jitter_ttl(cache_cfg),
        );
        let post_item_cache = DefaultCache::new(100, cache_jitter_ttl(cache_cfg));

        let post_create_interactor = PostCreateInteractor::new(
            conn.clone(),
            DefaultPostQuery::new(upload_file_url_builder.clone()),
            DefaultPostRepository,
            DefaultCategoryRepository,
            post_page_cache.clone(),
        )
        .into();

        let post_delete_interactor = PostDeleteInteractor::new(
            conn.clone(),
            DefaultPostRepository,
            post_page_cache.clone(),
            post_item_cache.clone(),
        )
        .into();

        let post_find_interactor = PostFindInteractor::new(
            conn.clone(),
            DefaultPostQuery::new(upload_file_url_builder.clone()),
            post_item_cache.clone(),
        )
        .into();

        let post_paginate_interactor = PostPaginateInteractor::new(
            conn.clone(),
            DefaultPostQuery::new(upload_file_url_builder.clone()),
            post_page_cache.clone(),
        )
        .into();

        let post_search_interactor = PostSearchInteractor::new(
            conn.clone(),
            DefaultPostQuery::new(upload_file_url_builder.clone()),
        )
        .into();

        let post_update_interactor = PostUpdateInteractor::new(
            conn.clone(),
            DefaultPostRepository,
            DefaultCategoryRepository,
            post_page_cache.clone(),
            post_item_cache.clone(),
        )
        .into();
        /////////////////////////////////////////////////////////////////

        let category_interactor = CategoryInteractor::new(
            conn.clone(),
            DefaultCategoryQuery,
            DefaultCategoryRepository,
            DefaultCache::new(1, cache_jitter_ttl(cache_cfg)),
            post_page_cache,
            post_item_cache,
        )
        .into();

        let attachment_interactor = AttachmentInteractor::new(
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
            post_create_interactor,
            post_delete_interactor,
            post_find_interactor,
            post_paginate_interactor,
            post_search_interactor,
            post_update_interactor,
            category_interactor,
            attachment_interactor,
        })
    }
}

////////////////////////////////////////////////////////////////////

impl FromRef<BloggingState> for Arc<Auth> {
    fn from_ref(input: &BloggingState) -> Self {
        input.auth.clone()
    }
}

fn cache_jitter_ttl(cache_cfg: &Cache) -> Duration {
    duration_with_jitter(
        cache_cfg.base_cache_ttl_sec,
        cache_cfg.base_cache_jitter_sec_range,
    )
}
