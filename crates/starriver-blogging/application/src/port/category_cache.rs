/// 分类列表缓存的 key 类型。
///
/// 用独立类型而不是 `()`，避免和其它 unit key 缓存混淆，
/// 也让 `state.rs` 的缓存类型声明能表达这个 key 的语义。
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct CategoryListKey;

pub const CACHE_KEY_CATEGORY_LIST: CategoryListKey = CategoryListKey;
