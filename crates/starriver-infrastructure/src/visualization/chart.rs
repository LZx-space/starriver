use anyhow::Error;
use std::fmt::Display;
use std::marker::PhantomData;

/// handle a query to producer a chart dataset
pub trait ChartHandler {
    type Query;

    type Row: Row;

    type Dataset: Dataset<Row = Self::Row>;

    fn handle(&self, ctx: Box<dyn Context<Query = Self::Query>>) -> Result<Self::Dataset, Error>;
}

/// implementing this trait, so u can pass more fields
pub trait Context {
    type Query;

    fn query(&self) -> &Self::Query;
}

pub trait Row {
    fn id(&self) -> &impl Display;
}

pub trait Dataset {
    type Row: Row;

    fn rows(&self) -> Vec<&Self::Row>;
}

// ----- impl ----------------------------------------------------

pub struct DefaultChartHandler<Q, R, D, F: Fn(Box<dyn Context<Query = Q>>) -> Result<D, Error>> {
    handler: F,
    _marker1: PhantomData<Q>,
    _marker2: PhantomData<R>,
    _marker3: PhantomData<D>,
}

impl<Q, R, D, F> DefaultChartHandler<Q, R, D, F>
where
    R: Row,
    D: Dataset<Row = R>,
    F: Fn(Box<dyn Context<Query = Q>>) -> Result<D, Error>,
{
    pub fn new(handler: F) -> Self {
        DefaultChartHandler {
            handler,
            _marker1: Default::default(),
            _marker2: Default::default(),
            _marker3: Default::default(),
        }
    }
}

impl<Q, R, D, F> ChartHandler for DefaultChartHandler<Q, R, D, F>
where
    R: Row,
    D: Dataset<Row = R>,
    F: Fn(Box<dyn Context<Query = Q>>) -> Result<D, Error>,
{
    type Query = Q;
    type Row = R;
    type Dataset = D;

    fn handle(&self, ctx: Box<dyn Context<Query = Self::Query>>) -> Result<Self::Dataset, Error> {
        (self.handler)(ctx)
    }
}

// ----- test ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        use std::fmt::Debug;
        use std::fmt::Formatter;
        use std::ops::Add;

        struct TestRow {
            id: String,

            column_1: String,
        }

        impl Row for TestRow {
            fn id(&self) -> &impl Display {
                &self.id
            }
        }

        // -----------------------------------

        struct TestQuery {}

        // ----------------------------------

        struct TestCtx {
            q: TestQuery,
        }

        impl Context for TestCtx {
            type Query = TestQuery;

            fn query(&self) -> &TestQuery {
                &self.q
            }
        }

        struct TestDataset {
            dateset: Vec<TestRow>,
        }

        impl Dataset for TestDataset {
            type Row = TestRow;

            fn rows(&self) -> Vec<&Self::Row> {
                self.dateset.iter().map(|e| e).collect()
            }
        }

        impl Debug for TestDataset {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let show = self
                    .dateset
                    .iter()
                    .map(|e| {
                        let x = &e.id;
                        let x1 = &e.column_1;
                        String::from(x).add("-").add(x1)
                    })
                    .reduce(|a, b| a.add(" ").add(b.as_str()));
                Ok(f.write_str(show.unwrap().as_str())?)
            }
        }

        // -----------------------------------

        let query = TestQuery {};
        let ctx = TestCtx { q: query };
        let handler = DefaultChartHandler::new(|_ctx| {
            Ok(TestDataset {
                dateset: vec![
                    TestRow {
                        id: "R1".to_string(),
                        column_1: "1".to_string(),
                    },
                    TestRow {
                        id: "R2".to_string(),
                        column_1: "2".to_string(),
                    },
                ],
            })
        });
        let result = handler.handle(Box::new(ctx));
        println!("----------{:?}", result.expect("fail to handle"));
    }
}
