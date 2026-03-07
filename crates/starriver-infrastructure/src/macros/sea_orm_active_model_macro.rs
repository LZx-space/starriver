/// 为了减少SQL操作字段；对比 ActiveModel 与聚合根（或实体）的属性值，不同才调用其 Set 方法
///
/// 参数说明：
/// - $active_model: 待更新字段值的ActiveModel
/// - $entity: 聚合根/实体的实例
/// - $($field:ident),*: 需要对比的字段列表
#[macro_export]
macro_rules! update_active_model_on_change {
    ($active_model:expr, $entity:expr, $($field:ident),*) => {{
        // 初始化标记：是否有字段执行了Set操作
        let mut any_updated = false;
        // 遍历指定字段，对比并设置 Set 状态
        $(
            // 对比聚合根字段与 ActiveModel 字段的值
            if !$entity.$field.eq($active_model.$field.as_ref()) {
                $active_model.$field = Set($entity.$field);
                // 有字段触发Set，标记为true
                any_updated = true;
            }
        )*
        // 返回是否有字段执行了Set操作
        any_updated
    }};
}
