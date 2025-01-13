use anyhow::Error;
use std::fmt::Display;

/// handle a query to producer a chart dataset
pub trait ChartHandler {
    type Query;

    type Row: Row;

    type Dataset: Dataset<Row = Self::Row>;

    fn handle(&self, ctx: impl Context<Query = Self::Query>) -> Result<Self::Dataset, Error>;
}

/// implementing this trait, so u can passing more fields
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

// ---------------------------------------------------------

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

    struct DummyQuery {}

    // ----------------------------------

    struct Ctx {
        q: DummyQuery,
    }

    impl Context for Ctx {
        type Query = DummyQuery;

        fn query(&self) -> &DummyQuery {
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
            f.write_str(show.unwrap().as_str()).unwrap();
            Ok(())
        }
    }

    // -----------------------------------

    struct TestChart {}

    impl ChartHandler for TestChart {
        type Query = DummyQuery;
        type Row = TestRow;
        type Dataset = TestDataset;

        fn handle(&self, _ctx: impl Context) -> Result<Self::Dataset, Error> {
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
        }
    }

    // -----------------------------------

    let handler = TestChart {};
    let query = DummyQuery {};
    let ctx = Ctx { q: query };
    let result = handler.handle(ctx);
    println!("----------{:?}", result.expect("fail to handle"));
}
