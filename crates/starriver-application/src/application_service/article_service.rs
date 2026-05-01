use crate::article_dto::req::{PageQuery, UpdateArticleCmd};
use crate::article_dto::res::ArticleAttachment;
use crate::dto::article_dto::res::{ArticleDetail, ArticleExcerpt};
use crate::query::article_query_service::{ArticleQueryService, DefaultArticleQueryService};
use crate::repository::article_repository::DefaultArticleRepository;
use infer::MatcherType;
use sea_orm::{DatabaseConnection, TransactionTrait};
use starriver_domain::article::domain_service::AttachmentService;
use starriver_domain::article::entity::{Article, Attachment};
use starriver_domain::article::params::ArticleUpdate;
use starriver_domain::article::repository::ArticleRepository;
use starriver_infrastructure::error::{ApiError, Cause};
use starriver_infrastructure::extract::Multipart;
use starriver_infrastructure::model::aggregate_revision::Revision;
use starriver_infrastructure::model::page::PageResult;
use starriver_infrastructure::security::authentication::_default_impl::AuthenticatedUser;
use starriver_infrastructure::service::config_service::Uploads;
use starriver_infrastructure::service::file_service::{delete_file, storage};

use std::path::PathBuf;
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct ArticleApplication {
    conn: DatabaseConnection,
    upload_cfg: Uploads,
    query: DefaultArticleQueryService,
    repo: DefaultArticleRepository<DatabaseConnection>,
}

impl ArticleApplication {
    /// 新建
    pub fn new(conn: DatabaseConnection, upload_cfg: Uploads) -> Self {
        let query = DefaultArticleQueryService { conn: conn.clone() };
        let repo = DefaultArticleRepository::new(conn.clone());
        Self {
            conn,
            upload_cfg,
            query,
            repo,
        }
    }

    pub async fn paginate(&self, q: PageQuery) -> Result<PageResult<ArticleExcerpt>, ApiError> {
        self.query.paginate(q).await
    }

    pub async fn find(&self, id: Uuid) -> Result<ArticleDetail, ApiError> {
        let mut article = self
            .query
            .find_detail(id)
            .await?
            .ok_or_else(|| ApiError::with_bad_request(format!("article[{id}] not exist")))?;
        // take 取出所有权，into_iter 避免 clone
        let rows = std::mem::take(&mut article.attachment_rows);
        article.attachments = rows
            .into_iter()
            .map(|e| {
                let file_name = e.file_name;
                let url = AttachmentService::access_url(&self.upload_cfg.proxy_prefix, &file_name);
                ArticleAttachment {
                    id: e.id,
                    file_name,
                    url,
                }
            })
            .collect();
        Ok(article)
    }

    pub async fn create_draft(&self, author: AuthenticatedUser) -> Result<ArticleDetail, ApiError> {
        let author_id = author.id;
        let draft_article = Article::new_empty_draft(author_id)?;
        let created = self.repo.add(draft_article).await?;
        let article_id = created.id().to_owned();
        self.find(article_id).await
    }

    pub async fn update(
        &self,
        operator: AuthenticatedUser,
        id: Uuid,
        cmd: UpdateArticleCmd,
    ) -> Result<(), ApiError> {
        info!(
            user_id = %operator.id,
            article_id = %id,
            "updating article"
        );
        let tx = self.conn.begin().await.map_err(ApiError::from)?;
        let tx_repo = DefaultArticleRepository::new(tx);
        match tx_update_article(id, cmd, &tx_repo).await {
            Ok(_) => {
                info!(article_id = %id, "article updated successfully");
                tx_repo.conn().commit().await?;
                Ok(())
            }
            Err(err) => {
                warn!(article_id = %id, error = %err, "article update failed, rolling back");
                tx_repo.conn().rollback().await?;
                Err(err)
            }
        }
    }

    /// 通常富文本编辑器展示一个附件需要上传
    pub async fn upload_attachment(
        &self,
        operator: AuthenticatedUser,
        article_id: Uuid,
        multipart: Multipart,
    ) -> Result<ArticleAttachment, ApiError> {
        info!(
            user_id = %operator.id,
            article_id = %article_id,
            "uploading attachment"
        );
        let paths = storage(multipart.0, &self.upload_cfg, vec![MatcherType::Image]).await?;
        // 解构为单元素数组，0 个或多个文件均报错
        let [file_path] = <[PathBuf; 1]>::try_from(paths)
            .map_err(|v| ApiError::with_bad_request(format!("expected 1 file, got {}", v.len())))?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| ApiError::with_inner_error("failed to extract filename from path"))?;

        let attachment = Attachment::new(article_id, file_name);
        let attachment_id = *attachment.id();
        // 开启事务, update方法内部会再次查询获取副本以对比更新字段，这依赖于事务等级
        let tx = self.conn.begin().await.map_err(ApiError::from)?;
        let tx_repo = DefaultArticleRepository::new(tx);
        match tx_upload_attachement(article_id, attachment, &tx_repo).await {
            Ok(_) => {
                tx_repo.conn().commit().await?;
                info!(
                    article_id = %article_id,
                    file_name = %file_name,
                    "upload attachment transaction successfully"
                );
                let url = AttachmentService::access_url(&self.upload_cfg.proxy_prefix, file_name);
                Ok(ArticleAttachment {
                    id: attachment_id,
                    file_name: file_name.to_owned(),
                    url,
                })
            }
            Err(e) => {
                error!(
                    article_id = %article_id,
                    file_name = %file_name,
                    error = %e,
                    "upload attachment transaction failed, rolling back and deleting file"
                );
                tx_repo.conn().rollback().await?;
                delete_file(&file_path).await?;
                info!(
                    article_id = %article_id,
                    file_name = %file_name,
                    "rolled back and deleted orphaned attachment file"
                );
                Err(e)
            }
        }
    }

    pub async fn delete_by_id(
        &self,
        operator: AuthenticatedUser,
        id: Uuid,
    ) -> Result<bool, ApiError> {
        info!(
            user_id = %operator.id,
            article_id = %id,
            "deleting article"
        );
        self.repo.delete_by_id(id).await
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
async fn tx_update_article(
    id: Uuid,
    cmd: UpdateArticleCmd,
    tx_repo: &DefaultArticleRepository<sea_orm::DatabaseTransaction>,
) -> Result<Article, ApiError> {
    let mut found = tx_repo.find_by_id(id).await?.ok_or_else(|| {
        ApiError::new(Cause::ClientBadRequest, format!("article[{}]not exist", id))
    })?;
    let cmd = ArticleUpdate {
        title: cmd.title,
        content: cmd.content,
        category_id: cmd.category_id,
        attachment_ids: cmd.attachment_ids,
        published: cmd.publish,
    };
    let original = found.clone();
    found.update(cmd)?;
    tx_repo.update(Revision::new(original, found)).await
}

/// # return
///   * 附件的URL/异常
async fn tx_upload_attachement(
    article_id: Uuid,
    attachment: Attachment,
    tx_repo: &DefaultArticleRepository<sea_orm::DatabaseTransaction>,
) -> Result<Article, ApiError> {
    // 将附件信息保存到博客中
    let mut found = tx_repo.find_by_id(article_id).await?.ok_or_else(|| {
        ApiError::new(
            Cause::ClientBadRequest,
            format!("article[{}]not exist", article_id),
        )
    })?;
    let original = found.clone();
    found.add_attachment(attachment)?;
    tx_repo.update(Revision::new(original, found)).await
}
