use uuid::Uuid;

/// 帖子分页查询缓存键，枚举具体参数而不用请求结构体避免添加额外参数时不适合做缓存而注意不到
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PostPageKey {
    pub page: u64,
    pub page_size: u64,
    pub published_only: bool,
    pub category_id: Option<Uuid>,
}
