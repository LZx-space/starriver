use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, DbErr, ExecResult,
    QueryResult, Statement, TransactionTrait,
};
use starriver_shared_base::db::{Connection, Executor, Transaction};

#[derive(Debug, Clone)]
pub struct DefaultConnection {
    inner: DatabaseConnection,
}

impl DefaultConnection {
    pub fn new(inner: DatabaseConnection) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &DatabaseConnection {
        &self.inner
    }
}

impl Connection for DefaultConnection {
    type Transaction = DefaultTransaction;
    type Error = DbErr;

    async fn begin(&self) -> Result<Self::Transaction, Self::Error> {
        self.inner.begin().await.map(Self::Transaction::new)
    }
}

impl Executor for DefaultConnection {}
impl ConnectionTrait for DefaultConnection {
    #[doc = " Fetch the database backend as specified in [DbBackend]."]
    #[doc = " This depends on feature flags enabled."]
    fn get_database_backend(&self) -> DbBackend {
        self.inner.get_database_backend()
    }

    #[doc = " Execute a [Statement]"]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn execute<'life0, 'async_trait>(
        &'life0 self,
        stmt: Statement,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<ExecResult, DbErr>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.inner.execute(stmt)
    }

    #[doc = " Execute a unprepared [Statement]"]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn execute_unprepared<'life0, 'life1, 'async_trait>(
        &'life0 self,
        sql: &'life1 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<ExecResult, DbErr>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.inner.execute_unprepared(sql)
    }

    #[doc = " Execute a [Statement] and return a query"]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn query_one<'life0, 'async_trait>(
        &'life0 self,
        stmt: Statement,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Option<QueryResult>, DbErr>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.inner.query_one(stmt)
    }

    #[doc = " Execute a [Statement] and return a collection Vec<[QueryResult]> on success"]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn query_all<'life0, 'async_trait>(
        &'life0 self,
        stmt: Statement,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Vec<QueryResult>, DbErr>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.inner.query_all(stmt)
    }
}

//////////////////////////////////////////////////////////////////////

pub struct DefaultTransaction {
    inner: DatabaseTransaction,
}

impl DefaultTransaction {
    pub fn new(inner: DatabaseTransaction) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &DatabaseTransaction {
        &self.inner
    }
}

impl Transaction for DefaultTransaction {
    type Error = DbErr;

    async fn commit(self) -> Result<(), Self::Error> {
        self.inner.commit().await
    }

    async fn rollback(self) -> Result<(), Self::Error> {
        self.inner.rollback().await
    }
}

impl Executor for DefaultTransaction {}
impl ConnectionTrait for DefaultTransaction {
    #[doc = " Fetch the database backend as specified in [DbBackend]."]
    #[doc = " This depends on feature flags enabled."]
    fn get_database_backend(&self) -> DbBackend {
        self.inner.get_database_backend()
    }

    #[doc = " Execute a [Statement]"]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn execute<'life0, 'async_trait>(
        &'life0 self,
        stmt: Statement,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<ExecResult, DbErr>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.inner.execute(stmt)
    }

    #[doc = " Execute a unprepared [Statement]"]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn execute_unprepared<'life0, 'life1, 'async_trait>(
        &'life0 self,
        sql: &'life1 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<ExecResult, DbErr>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.inner.execute_unprepared(sql)
    }

    #[doc = " Execute a [Statement] and return a query"]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn query_one<'life0, 'async_trait>(
        &'life0 self,
        stmt: Statement,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Option<QueryResult>, DbErr>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.inner.query_one(stmt)
    }

    #[doc = " Execute a [Statement] and return a collection Vec<[QueryResult]> on success"]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn query_all<'life0, 'async_trait>(
        &'life0 self,
        stmt: Statement,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Vec<QueryResult>, DbErr>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.inner.query_all(stmt)
    }
}
