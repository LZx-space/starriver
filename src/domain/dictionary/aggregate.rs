use std::any::{Any, TypeId};
use std::fmt::Error;

pub struct Dictionary {
    entry_vec: Vec<DictionaryEntry>,
}

impl Dictionary {}

pub struct DictionaryEntry {
    type_id: TypeId,
    value: String,
}

impl DictionaryEntry {
    pub fn new(type_id: TypeId, value: String) -> Result<Self, Error> {
        // validate
        Ok(DictionaryEntry { type_id, value })
    }

    fn parse(&self) -> Box<dyn Any> {
        let i = self.value.parse::<i32>().expect("12313");
        Box::new(i)
    }
}

#[test]
fn test_de() {
    let entry = DictionaryEntry::new(TypeId::of::<bool>(), "0".to_string());
    println!("------------------1");
    let n = entry.expect("").parse();
    println!("------------------{:?}", n);
}
