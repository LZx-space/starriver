/// 排序方向
#[allow(unused)]
pub enum Direction {
    ASC,

    DESC,
}

/// 依据哪个成员变量做何种顺序的排序
#[allow(unused)]
pub struct Order<'a> {
    property: &'a String,

    direction: Direction,
}

/// 结合各Order结构体做综合的排序
#[allow(unused)]
pub struct Sort<'a> {
    orders: Vec<Order<'a>>,
}
