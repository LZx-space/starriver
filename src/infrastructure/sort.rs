pub enum Direction {
    ASC,

    DESC,
}

///
pub struct Order<'a> {
    property: &'a str,

    direction: Direction,
}

///
pub struct Sort<'a> {
    orders: Vec<Order<'a>>,
}
