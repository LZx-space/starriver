use std::path::PathBuf;

use crate::article_dto::req::{ArticleAttachmentCmd, ArticleCmd};
use crate::dto::article_dto::res::{ArticleDetail, ArticleExcerpt};
use crate::query::article_query_service::{ArticleQueryService, DefaultArticleQueryService};
use crate::repository::article_repository::DefaultArticleRepository;
use sea_orm::{DatabaseConnection, TransactionTrait};
use starriver_domain::article::entity::{Article, Attachment};
use starriver_domain::article::params::ArticleUpdate;
use starriver_domain::article::repository::ArticleRepository;
use starriver_infrastructure::error::{ApiError, Cause};
use starriver_infrastructure::model::page::{PageQuery, PageResult};
use starriver_infrastructure::security::authentication::_default_impl::AuthenticatedUser;
use starriver_infrastructure::service::config_service::Assets;
use starriver_infrastructure::service::file_service::{delete_file, write_to_file};
use starriver_infrastructure::util::db::TransactionalConn;
use tracing::{error, info};
use uuid::Uuid;

pub struct ArticleApplication {
    conn: DatabaseConnection,
    static_cfg: Assets,
    query: DefaultArticleQueryService,
    repo: DefaultArticleRepository<DatabaseConnection>,
}

impl ArticleApplication {
    /// 新建
    pub fn new(conn: DatabaseConnection, static_cfg: Assets) -> Self {
        let query = DefaultArticleQueryService { conn: conn.clone() };
        let repo = DefaultArticleRepository::new(conn.clone());
        Self {
            conn,
            static_cfg,
            query,
            repo,
        }
    }

    pub async fn page(&self, q: PageQuery) -> Result<PageResult<ArticleExcerpt>, ApiError> {
        self.query.find_page(q).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<ArticleDetail, ApiError> {
        find_article_by_id(&self.repo, id).await.map(Into::into)
    }

    pub async fn add_empty_draft(
        &self,
        author: AuthenticatedUser,
    ) -> Result<ArticleDetail, ApiError> {
        let author_id = author.id;
        let draft_article = Article::new_empty_draft(author_id)?;
        self.repo.add(draft_article).await.map(Into::into)
    }

    pub async fn update(
        &self,
        operator: AuthenticatedUser,
        id: Uuid,
        cmd: ArticleCmd,
    ) -> Result<ArticleDetail, ApiError> {
        info!("用户[{}]更新博客[{}]", operator.username, id);
        // 开启事务, update方法内部会再次查询获取副本以对比更新字段，这依赖于事务等级
        let tx = self.conn.begin().await.map_err(ApiError::from)?;
        let tx_repo = DefaultArticleRepository::new(tx);
        match Self::tx_update_article(id, cmd, &tx_repo).await {
            Ok(article) => {
                info!("博客[{}]更新成功", id);
                // 提交事务
                tx_repo.conn().commit().await?;
                // todo 离线删除不用的附件
                Ok(article)
            }
            Err(err) => {
                info!("博客[{}]更新失败: {}", id, err);
                // 回滚事务
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
        file: ArticleAttachmentCmd,
    ) -> Result<String, ApiError> {
        info!("用户[{}]为博客[{}]添加附件", operator.username, article_id);
        // 检查文件格式
        let extension = match infer::get(&file.data).map(|t| t.extension()) {
            Some(e) => e,
            None => {
                return Err(ApiError::with_bad_request("错误的文件格式"));
            }
        };
        info!("user upload file actual extension: {}", extension);
        if extension != file.extension {
            return Err(ApiError::with_bad_request("文件格式与文件名不匹配"));
        }
        let attachment = Attachment::new(article_id);
        // 使用附件ID作为文件名，以便后续定位做其他操作
        let file_name = attachment.filename(&file.extension);
        let file_name = file_name.as_str();
        // 从配置文件获取上传目录
        let target_dir = Self::upload_dir(&self.static_cfg)?;
        // 写入数据
        write_to_file(target_dir.as_path(), file_name, file.data).await?;
        info!("博客[{}]保存附件[{}]成功", article_id, file_name);

        // 开启事务, update方法内部会再次查询获取副本以对比更新字段，这依赖于事务等级
        let tx = self.conn.begin().await.map_err(ApiError::from)?;
        let tx_repo = DefaultArticleRepository::new(tx);
        match Self::tx_upload_attachement(article_id, attachment, &tx_repo).await {
            Ok(_) => {
                info!("博客[{}]附件[{}]添加成功", article_id, file_name);
                // 提交事务
                tx_repo.conn().commit().await?;
                let url = format!("{}/{}", &self.static_cfg.uploads.relative_dir, file_name);
                Ok(url)
            }
            Err(e) => {
                error!("博客[{}]添加附件[{}]失败: {}", article_id, file_name, e);
                // 回滚事务
                tx_repo.conn().rollback().await?;
                // 删除附件
                delete_file(target_dir.as_path(), file_name).await?;
                info!("事务回滚，博客[{}]删除附件[{}]成功", article_id, file_name);
                Err(e)
            }
        }
    }

    pub async fn delete_by_id(
        &self,
        operator: AuthenticatedUser,
        id: Uuid,
    ) -> Result<bool, ApiError> {
        info!("用户[{}]删除博客[{}]", operator.username, id);
        self.repo.delete_by_id(id).await
    }

    //////////////////////////////////////////////////////////////////////////////////////

    async fn tx_update_article(
        id: Uuid,
        cmd: ArticleCmd,
        tx_repo: &DefaultArticleRepository<sea_orm::DatabaseTransaction>,
    ) -> Result<ArticleDetail, ApiError> {
        let mut found = find_article_by_id(tx_repo, id).await?;
        let cmd = ArticleUpdate {
            title: cmd.title,
            content: cmd.content,
            attachment_ids: cmd.attachment_ids,
            published: cmd.publish,
        };
        found.update(cmd)?;
        tx_repo.update(found).await.map(Into::into)
    }

    /// # return
    ///   * 附件的URL/异常
    async fn tx_upload_attachement(
        article_id: Uuid,
        attachment: Attachment,
        tx_repo: &DefaultArticleRepository<sea_orm::DatabaseTransaction>,
    ) -> Result<Article, ApiError> {
        // 将附件信息保存到博客中
        let mut found = find_article_by_id(tx_repo, article_id).await?;
        found.add_attachment(attachment)?;
        tx_repo.update(found).await
    }

    fn upload_dir(cfg: &Assets) -> Result<PathBuf, ApiError> {
        let static_base_dir = &cfg.static_base_dir.as_str();
        let upload_dir = &cfg.uploads.relative_dir.as_str();
        let target_dir = format!("{}/{}", static_base_dir, upload_dir);
        let target_dir = target_dir
            .parse::<PathBuf>()
            .map_err(|e| ApiError::with_inner_error(e.to_string()))?;
        Ok(target_dir)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////

async fn find_article_by_id(
    repo: &DefaultArticleRepository<impl TransactionalConn>,
    id: Uuid,
) -> Result<Article, ApiError> {
    repo.find_by_id(id)
        .await?
        .ok_or_else(|| ApiError::new(Cause::ClientBadRequest, format!("博客{}不存在", id)))
}
