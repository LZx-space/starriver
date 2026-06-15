use starriver_shared_base::{cache::Cache, dto::PageResult};
use uuid::Uuid;

use crate::dto::post_dto::res::{PostDetailDto, PostExcerptDto};

pub struct PostCaches<PC, DC> {
    page_cache: PC,
    detail_cache: DC,
}

impl<PC, DC> PostCaches<PC, DC>
where
    PC: Cache<PostPageKey, PageResult<PostExcerptDto>>,
    DC: Cache<Uuid, Option<PostDetailDto>>,
{
    pub fn new(page_cache: PC, detail_cache: DC) -> Self {
        Self {
            page_cache,
            detail_cache,
        }
    }

    pub fn page_cache(&self) -> &PC {
        &self.page_cache
    }

    pub fn detail_cache(&self) -> &DC {
        &self.detail_cache
    }

    /// 增删改后统一清除所有帖子相关缓存
    pub fn invalidate_all(&self) {
        self.page_cache.invalidate_all();
        self.detail_cache.invalidate_all();
    }
}

/// 帖子分页查询缓存键，枚举具体参数而不用请求结构体避免添加额外参数时不适合做缓存而注意不到
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PostPageKey {
    pub page: u64,
    pub page_size: u64,
    pub published_only: bool,
    pub category_id: Option<Uuid>,
}
