use std::str::FromStr;

pub struct Dictionary {
    type_id: TypeId,
    value: String,
}

impl Dictionary {
    fn parse<T>(&self, t: T) -> T
    where
        T: FromStr,
    {
        let result: Result<T, _> = self.value.parse();
        todo!()
    }
}

pub enum TypeId {}

pub struct DictionaryItem {}
