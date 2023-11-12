use std::marker::PhantomData;

pub struct Charts<Q, T, U> {
    result_handler: U,
    _marker1: PhantomData<Q>,
    _marker2: PhantomData<T>,
}

impl<Q, T, U> Charts<Q, T, U>
where
    Q: RowQuery,
    T: Result<Row = Q::Row>,
    U: ResultHandler<Row = Q::Row, Result = T>,
{
    fn handle(&self, query: Q) -> T {
        let default_rows = query.default_rows();
        self.result_handler.handle(default_rows)
    }
}

pub trait Result {
    type Row: Row;

    fn dateset(&self) -> Vec<Self::Row>;
}

pub trait Row: Default {
    fn label(&self) -> &str;
}

pub trait RowQuery {
    type Row: Row;

    fn default_rows(&self) -> Vec<Self::Row>;
}

pub trait ResultHandler {
    type Row: Row;

    type Result: Result<Row = Self::Row>;
    fn handle(&self, default_rows: Vec<Self::Row>) -> Self::Result;
}

#[test]
pub fn test() {}
