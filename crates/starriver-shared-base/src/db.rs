use std::fmt::Display;

/// sql executor
pub trait Executor {}

/// an executor that can create transactions
pub trait Connection: Executor {
    type Transaction: Transaction;
    type Error: Display;
    fn begin(&self) -> impl Future<Output = Result<Self::Transaction, Self::Error>>;
}

/// an executor that can commit or rollback transactions
pub trait Transaction: Executor {
    type Error: Display;

    fn commit(self) -> impl Future<Output = Result<(), Self::Error>>;
    fn rollback(self) -> impl Future<Output = Result<(), Self::Error>>;
}

//////////////////////////////////////////////////////////////////////////////////////

pub struct Revision<T: Clone> {
    /// 变更前（原始、旧版本）
    original: T,
    /// 变更后（当前、新版本）
    modified: T,
}

impl<T> Revision<T>
where
    T: Clone,
{
    pub fn new(original: T, modified: T) -> Self {
        Self { original, modified }
    }

    pub fn dissolve(self) -> (T, T) {
        (self.original, self.modified)
    }
}
