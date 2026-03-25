use sea_orm::{ConnectionTrait, TransactionTrait};

/// 表明既可以是普通链接，也可以是事务链接（使用完需要马上提交，且不能共享）
pub trait TransactionalConn: TransactionTrait + ConnectionTrait + Send + Sync {}
impl<T: TransactionTrait + ConnectionTrait + Send + Sync> TransactionalConn for T {}
