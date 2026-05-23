use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use starriver_blogging_application::service::{
    attachement_service::AttachmentApplication, category_service::CategoryApplication,
    post_service::PostApplication,
};
use starriver_blogging_domain::attachment::factory::AttachmentFactory;
use starriver_shared_framework::{
    config::{Auth, Uploads},
    file_access::DefaultUploadLocationResolver,
};

use crate::port_out::{
    attachment_repository::DefaultAttachmentRepository,
    category_repository::DefaultCategoryRepository, file_type_checker::DefaultFileTypeChecker,
    post_query_port::DefaultPostQueryPort, post_repository::DefaultPostRepository,
};

/// 应用的各个状态
#[derive(Clone)]
pub struct BloggingState {
    pub auth: Auth,
    pub uploads: Uploads,
    pub upload_file_url_builder: Arc<DefaultUploadLocationResolver>,
    pub post_service: Arc<PostApplication<DefaultPostQueryPort, DefaultPostRepository>>,
    pub category_service: Arc<CategoryApplication<DefaultCategoryRepository>>,
    pub attachment_service: Arc<
        AttachmentApplication<
            DefaultAttachmentRepository,
            DefaultFileTypeChecker,
            DefaultUploadLocationResolver,
        >,
    >,
}

impl BloggingState {
    pub async fn new(
        conn: DatabaseConnection,
        auth: Auth,
        uploads: Uploads,
    ) -> Result<Self, String> {
        let upload_file_url_builder = Arc::new(DefaultUploadLocationResolver::new(uploads.clone()));
        let post_service = PostApplication::new(
            DefaultPostQueryPort::new(conn.clone(), upload_file_url_builder.clone()),
            DefaultPostRepository::new(conn.clone()),
        )
        .into();
        let category_service =
            CategoryApplication::new(DefaultCategoryRepository::new(conn.clone())).into();
        let attachment_service = AttachmentApplication::new(
            DefaultAttachmentRepository::new(conn.clone()),
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

impl FromRef<BloggingState> for Auth {
    fn from_ref(input: &BloggingState) -> Self {
        input.auth.clone()
    }
}
