use std::path::PathBuf;

use crate::blog_dto::req::{BlogAttachmentCmd, BlogCmd};
use crate::dto::blog_dto::res::{BlogDetail, BlogExcerpt};
use crate::query::blog_query_service::{BlogQueryService, DefaultBlogQueryService};
use crate::repository::blog_repository::DefaultBlogRepository;
use sea_orm::{DatabaseConnection, TransactionTrait};
use starriver_domain::blog::entity::{Attachment, Blog};
use starriver_domain::blog::params::BlogUpdate;
use starriver_domain::blog::repository::BlogRepository;
use starriver_infrastructure::error::{ApiError, Cause};
use starriver_infrastructure::model::page::{PageQuery, PageResult};
use starriver_infrastructure::security::authentication::_default_impl::AuthenticatedUser;
use starriver_infrastructure::service::config_service::Assets;
use starriver_infrastructure::service::file_service::{delete_file, write_to_file};
use starriver_infrastructure::util::db::TransactionalConn;
use tracing::{error, info};
use uuid::Uuid;

pub struct BlogApplication {
    conn: DatabaseConnection,
    static_cfg: Assets,
    query: DefaultBlogQueryService,
    repo: DefaultBlogRepository<DatabaseConnection>,
}

impl BlogApplication {
    /// 新建
    pub fn new(conn: DatabaseConnection, static_cfg: Assets) -> Self {
        let query = DefaultBlogQueryService { conn: conn.clone() };
        let repo = DefaultBlogRepository::new(conn.clone());
        Self {
            conn,
            static_cfg,
            query,
            repo,
        }
    }

    pub async fn page(&self, q: PageQuery) -> Result<PageResult<BlogExcerpt>, ApiError> {
        self.query.find_page(q).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<BlogDetail, ApiError> {
        find_blog_by_id(&self.repo, id).await.map(Into::into)
    }

    pub async fn add_empty_draft(&self, author: AuthenticatedUser) -> Result<BlogDetail, ApiError> {
        let author_id = author.id;
        let draft_blog = Blog::new_empty_draft(author_id)?;
        self.repo.add(draft_blog).await.map(Into::into)
    }

    pub async fn update(
        &self,
        operator: AuthenticatedUser,
        id: Uuid,
        cmd: BlogCmd,
    ) -> Result<BlogDetail, ApiError> {
        info!("用户[{}]更新博客[{}]", operator.username, id);
        // 开启事务, update方法内部会再次查询获取副本以对比更新字段，这依赖于事务等级
        let tx = self.conn.begin().await.map_err(ApiError::from)?;
        let tx_repo = DefaultBlogRepository::new(tx);
        match Self::tx_update_blog(id, cmd, &tx_repo).await {
            Ok(blog) => {
                info!("博客[{}]更新成功", id);
                // 提交事务
                tx_repo.conn().commit().await?;
                // todo 离线删除不用的附件
                Ok(blog)
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
        blog_id: Uuid,
        file: BlogAttachmentCmd,
    ) -> Result<String, ApiError> {
        info!("用户[{}]为博客[{}]添加附件", operator.username, blog_id);
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
        let attachment = Attachment::new(blog_id);
        // 使用附件ID作为文件名，以便后续定位做其他操作
        let file_name = attachment.filename(&file.extension);
        let file_name = file_name.as_str();
        // 从配置文件获取上传目录
        let target_dir = Self::upload_dir(&self.static_cfg)?;
        // 写入数据
        write_to_file(target_dir.as_path(), file_name, file.data).await?;
        info!("博客[{}]保存附件[{}]成功", blog_id, file_name);

        // 开启事务, update方法内部会再次查询获取副本以对比更新字段，这依赖于事务等级
        let tx = self.conn.begin().await.map_err(ApiError::from)?;
        let tx_repo = DefaultBlogRepository::new(tx);
        match Self::tx_upload_attachement(blog_id, attachment, &tx_repo).await {
            Ok(_) => {
                info!("博客[{}]附件[{}]添加成功", blog_id, file_name);
                // 提交事务
                tx_repo.conn().commit().await?;
                let url = format!("{}/{}", &self.static_cfg.uploads.relative_dir, file_name);
                Ok(url)
            }
            Err(e) => {
                error!("博客[{}]添加附件[{}]失败: {}", blog_id, file_name, e);
                // 回滚事务
                tx_repo.conn().rollback().await?;
                // 删除附件
                delete_file(target_dir.as_path(), file_name).await?;
                info!("事务回滚，博客[{}]删除附件[{}]成功", blog_id, file_name);
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

    async fn tx_update_blog(
        id: Uuid,
        cmd: BlogCmd,
        tx_repo: &DefaultBlogRepository<sea_orm::DatabaseTransaction>,
    ) -> Result<BlogDetail, ApiError> {
        let mut found_blog = find_blog_by_id(tx_repo, id).await?;
        let cmd = BlogUpdate {
            title: cmd.title,
            content: cmd.content,
            attachment_ids: cmd.attachment_ids,
            published: cmd.publish,
        };
        found_blog.update(cmd)?;
        tx_repo.update(found_blog).await.map(Into::into)
    }

    /// # return
    ///   * 附件的URL/异常
    async fn tx_upload_attachement(
        blog_id: Uuid,
        attachment: Attachment,
        tx_repo: &DefaultBlogRepository<sea_orm::DatabaseTransaction>,
    ) -> Result<Blog, ApiError> {
        // 将附件信息保存到博客中
        let mut found_blog = find_blog_by_id(tx_repo, blog_id).await?;
        found_blog.add_attachment(attachment)?;
        tx_repo.update(found_blog).await
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

async fn find_blog_by_id(
    repo: &DefaultBlogRepository<impl TransactionalConn>,
    id: Uuid,
) -> Result<Blog, ApiError> {
    repo.find_by_id(id)
        .await?
        .ok_or_else(|| ApiError::new(Cause::ClientBadRequest, format!("博客{}不存在", id)))
}
