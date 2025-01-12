use std::marker::PhantomData;

pub struct Charts<Q, R, T, U> {
    result_handler: U,
    _marker1: PhantomData<Q>,
    _marker2: PhantomData<R>,
    _marker3: PhantomData<T>,
}

impl<Q, R, T, U> Charts<Q, R, T, U>
where
    T: Result<Row = R>,
    U: QueryHandler<Query = Q, Row = R, Result = T>,
{
    fn new(result_handler: U) -> Self {
        Charts {
            result_handler,
            _marker1: Default::default(),
            _marker2: Default::default(),
            _marker3: Default::default(),
        }
    }

    fn handle(&self, context: Context<Q>) -> T {
        self.result_handler.handle(context)
    }
}

pub struct Context<Q> {
    query: Q,
}

pub trait Row {
    fn id(&self) -> &str;
}

pub trait Result {
    type Row: Row;

    fn dateset(&self) -> Vec<&Self::Row>;
}

pub trait QueryHandler {
    type Query;

    type Row: Row;

    type Result: Result<Row = Self::Row>;

    fn handle(&self, context: Context<Self::Query>) -> Self::Result;
}

#[test]
pub fn test() {
    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::ops::Add;

    struct TestRow {
        id: String,

        column_1: String,
    }

    impl TestRow {
        fn new(label: &str) -> Self {
            TestRow {
                id: String::from(label),
                column_1: "123".to_string(),
            }
        }
    }

    impl Row for TestRow {
        fn id(&self) -> &str {
            &self.id
        }
    }

    // -----------------------------------

    struct TestRowQuery {}

    // ----------------------------------

    struct TestResult {
        dateset: Vec<TestRow>,
    }

    impl Result for TestResult {
        type Row = TestRow;

        fn dateset(&self) -> Vec<&Self::Row> {
            self.dateset.iter().map(|e| e).collect()
        }
    }

    impl Debug for TestResult {
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

    struct TestResultHandler {}

    impl QueryHandler for TestResultHandler {
        type Query = TestRowQuery;
        type Row = TestRow;
        type Result = TestResult;

        fn handle(&self, context: Context<Self::Query>) -> Self::Result {
            TestResult {
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
            }
        }
    }

    // -----------------------------------

    let handler = TestResultHandler {};
    let charts = Charts::new(handler);
    let query = TestRowQuery {};
    let context = Context { query };
    let result = charts.handle(context);
    println!("{:?}", result)
}
