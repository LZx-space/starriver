use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use starriver_blogging_application::use_case::{
    attachement_interactor::AttachmentApplication, category_interactor::CategoryApplication,
    post_interactor::PostApplication,
};
use starriver_blogging_domain::attachment::factory::AttachmentFactory;
use starriver_shared_framework::{
    config::{Auth, Uploads},
    upload_file::DefaultUploadLocationResolver,
};
use std::sync::Arc;

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
    pub upload_file_url_builder: DefaultUploadLocationResolver,
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
        let upload_file_url_builder = DefaultUploadLocationResolver::new(uploads.clone());
        let post_service = PostApplication::new(
            conn.clone(),
            DefaultPostQuery::new(upload_file_url_builder.clone()),
            DefaultPostRepository,
        )
        .into();

        let category_service = CategoryApplication::new(
            conn.clone(),
            DefaultCategoryQuery,
            DefaultCategoryRepository,
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
