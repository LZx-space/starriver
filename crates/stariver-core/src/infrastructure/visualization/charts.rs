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
    fn new(result_handler: U) -> Self {
        Charts {
            result_handler,
            _marker1: Default::default(),
            _marker2: Default::default(),
        }
    }

    fn handle(&self, query: Q) -> T {
        let initialized_rows = query.initialized_rows();
        self.result_handler.update(initialized_rows)
    }
}

pub trait Row {
    fn label(&self) -> &str;
}

pub trait RowQuery {
    type Row: Row;

    fn initialized_rows(&self) -> Vec<Self::Row>;
}

pub trait Result {
    type Row: Row;

    fn dateset(&self) -> Vec<Self::Row>;
}

pub trait ResultHandler {
    type Row: Row;

    type Result: Result<Row = Self::Row>;

    fn update(&self, initialized_rows: Vec<Self::Row>) -> Self::Result;
}

#[test]
pub fn test() {
    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::ops::Add;

    struct TestRow {
        label: String,

        column_1: String,
    }

    impl TestRow {
        fn new(label: &str) -> Self {
            TestRow {
                label: String::from(label),
                column_1: "123".to_string(),
            }
        }
    }

    impl Row for TestRow {
        fn label(&self) -> &str {
            &self.label
        }
    }

    // -----------------------------------

    struct TestRowQuery {}

    impl RowQuery for TestRowQuery {
        type Row = TestRow;

        fn initialized_rows(&self) -> Vec<Self::Row> {
            vec![TestRow::new("行A"), TestRow::new("行B")]
        }
    }

    // ----------------------------------

    struct TestResult {
        dateset: Vec<TestRow>,
    }

    impl Result for TestResult {
        type Row = TestRow;

        fn dateset(&self) -> Vec<Self::Row> {
            self.dateset()
        }
    }

    impl Debug for TestResult {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let show = self
                .dateset
                .iter()
                .map(|e| {
                    let x = &e.label;
                    let x1 = &e.column_1;
                    String::from(x).add("-").add(x1)
                })
                .reduce(|mut a, b| a.add(" ").add(b.as_str()));
            f.write_str(show.unwrap().as_str()).unwrap();
            Ok(())
        }
    }

    // -----------------------------------

    struct TestResultHandler {}

    impl ResultHandler for TestResultHandler {
        type Row = TestRow;
        type Result = TestResult;

        fn update(&self, mut initialized_rows: Vec<Self::Row>) -> Self::Result {
            let vec: Vec<TestRow> = initialized_rows
                .iter_mut()
                .map(|e| TestRow {
                    label: e.label.clone(),
                    column_1: "ACV".to_string(),
                })
                .collect();
            TestResult { dateset: vec }
        }
    }

    // -----------------------------------

    let handler = TestResultHandler {};
    let charts = Charts::new(handler);
    let query = TestRowQuery {};
    let result = charts.handle(query);
    println!("{:?}", result)
}
